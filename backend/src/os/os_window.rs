use crate::errors::AppError;
use crate::models::device::UsbDevice;

#[cfg(target_os = "windows")]
use winreg::{RegKey, enums::*};

#[cfg(target_os = "windows")]
pub async fn get_history_usb_from_os() -> Result<Vec<UsbDevice>, AppError> {
    let mut usb_devices = Vec::new();

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    // Меняем путь на USBSTOR — здесь лежат только флешки, HDD и SSD
    let usbstor_path = r"SYSTEM\CurrentControlSet\Enum\USBSTOR";

    let usbstor_key = match hklm.open_subkey_with_flags(usbstor_path, KEY_READ) {
        Ok(key) => key,
        Err(e) => {
            return Err(AppError::Validation(format!(
                "Ошибка доступа к USBSTOR: {}",
                e
            )));
        }
    };

    // 1. Перебираем папки устройств (например: "Disk&Ven_Kingston&Prod_DataTraveler_3.0&Rev_PMAP")
    for device_type_res in usbstor_key.enum_keys() {
        let device_type_str = match device_type_res {
            Ok(name) => name,
            Err(_) => continue,
        };

        // Извлекаем красивое имя производителя и модели из названия папки, если FriendlyName не заполнится
        let fallback_manufacturer = parse_usbstor_name(&device_type_str);

        if let Ok(device_key) = usbstor_key.open_subkey_with_flags(&device_type_str, KEY_READ) {
            // 2. Перебираем серийные номера конкретных флешек внутри этой модели
            for serial_res in device_key.enum_keys() {
                let serial_str = match serial_res {
                    Ok(name) => name,
                    Err(_) => continue,
                };

                if let Ok(instance_key) = device_key.open_subkey_with_flags(&serial_str, KEY_READ) {
                    // Читаем свойства, заполненные Windows
                    let manufacturer: String = instance_key.get_value("Mfg").unwrap_or_default();
                    let friendly_name: String =
                        instance_key.get_value("FriendlyName").unwrap_or_default();

                    // Формируем итоговое имя флешки
                    let name = if !friendly_name.is_empty() {
                        friendly_name
                    } else if !manufacturer.is_empty() {
                        manufacturer
                    } else {
                        fallback_manufacturer.clone()
                    };

                    // Очищаем серийный номер от суффиксов ревизии, если Windows их добавила (например, "&0")
                    let final_serial = match serial_str.split('&').next() {
                        Some(s) => s.to_string(),
                        None => serial_str.clone(),
                    };

                    // В USBSTOR нет VID/PID в названии папки, но Windows часто дублирует их
                    // в свойствах "HardwareID" или "CompatibleIDs" в виде массива строк.
                    // Извлечем их из совместимых ID или оставим None для ручного сопоставления.
                    let (vid, pid) = match instance_key.get_value::<Vec<String>, _>("HardwareID") {
                        Ok(ids) => parse_vid_pid_from_hardware_ids(&ids),
                        Err(_) => (None, None),
                    };

                    usb_devices.push(UsbDevice {
                        manufacturer: name,
                        serial: final_serial,
                        filesystem: None, // Для файловой системы нужен анализ ветки SOFTWARE (WPD)
                        capacity: None,
                        vid,
                        pid,
                    });
                }
            }
        }
    }

    Ok(usb_devices)
}

// Помощник для парсинга папки вида "Disk&Ven_Kingston&Prod_DataTraveler_3.0&Rev_PMAP"
fn parse_usbstor_name(folder_name: &str) -> String {
    let mut vendor = "";
    let mut prod = "";

    for part in folder_name.split('&') {
        if part.starts_with("Ven_") {
            vendor = &part[4..];
        } else if part.starts_with("Prod_") {
            prod = &part[5..];
        }
    }

    if vendor.is_empty() && prod.is_empty() {
        folder_name.to_string()
    } else {
        format!("{} {}", vendor.replace('_', " "), prod.replace('_', " "))
            .trim()
            .to_string()
    }
}

// Помощник для поиска VID/PID в системных HardwareID флешки
fn parse_vid_pid_from_hardware_ids(ids: &[String]) -> (Option<u16>, Option<u16>) {
    for id in ids {
        let id_upper = id.to_uppercase();
        if id_upper.contains("VID_") && id_upper.contains("PID_") {
            let vid = id_upper
                .split("VID_")
                .nth(1)
                .and_then(|s| s.split('&').next())
                .and_then(|hex| u16::from_str_radix(hex, 16).ok());

            let pid = id_upper
                .split("PID_")
                .nth(1)
                .and_then(|s| s.split('&').next())
                .and_then(|hex| u16::from_str_radix(hex, 16).ok());

            return (vid, pid);
        }
    }
    (None, None)
}

#[cfg(target_os = "windows")]
use windows::Win32::Devices::DeviceAndDriverInstallation::HDEVINFO;
use windows::Win32::Devices::DeviceAndDriverInstallation::*;
use windows::core::{GUID, PCWSTR};


/// Находит подключённые USB-флешки (массовые накопители)
pub async fn get_current_usb_flash_drives() -> Result<Vec<UsbDevice>, AppError> {
    let mut result = Vec::new();

    unsafe {
        // GUID интерфейса дисков: {53F56307-B6BF-11D0-94F2-00A0C91EFB8B}
        let guid_disk = GUID::from_u128(0x53F56307_B6BF_11D0_94F2_00A0C91EFB8B);

        // Перебираем все диски в системе (включая USB-флешки)
        let device_info_set = SetupDiGetClassDevsW(
            Some(&guid_disk),
            PCWSTR::null(),
            None,
            DIGCF_PRESENT | DIGCF_DEVICEINTERFACE,
        )
        .map_err(|e| AppError::Validation(format!("SetupDiGetClassDevs: {:?}", e)))?;

        let mut index = 0u32;

        loop {
            let mut dev_info = SP_DEVINFO_DATA {
                cbSize: std::mem::size_of::<SP_DEVINFO_DATA>() as u32,
                ..Default::default()
            };

            // Перебираем устройства по индексу
            if SetupDiEnumDeviceInfo(device_info_set, index, &mut dev_info).is_err() {
                break; // больше устройств нет
            }

            // Проверяем, является ли родитель этого диска USB-устройством
            if is_usb_device(device_info_set, &dev_info) {
                if let Some(device) = extract_disk_info(device_info_set, &dev_info) {
                    result.push(device);
                }
            }

            index += 1;
        }

        let _ = SetupDiDestroyDeviceInfoList(device_info_set);
    }

    Ok(result)
}

/// Проверяет, подключён ли диск через USB, поднимаясь по цепочке родителей
unsafe fn is_usb_device(device_info_set: HDEVINFO, dev_info: &SP_DEVINFO_DATA) -> bool {
    unsafe {
        // Получаем текущий instance ID
        let mut id_buffer = [0u16; 512];
        let mut required_size = 0u32;

        if SetupDiGetDeviceInstanceIdW(
            device_info_set,
            dev_info,
            Some(&mut id_buffer),
            Some(&mut required_size),
        )
        .is_err()
        {
            return false;
        }

        let instance_id =
            String::from_utf16_lossy(&id_buffer[..(required_size as usize).saturating_sub(1)]);

        // Если это USBSTOR — это точно USB-флешка
        if instance_id.starts_with("USBSTOR") {
            return true;
        }

        // Если это USB\ — тоже подходит
        if instance_id.starts_with("USB\\") {
            return true;
        }

        false
    }
}

/// Извлекает данные диска (имя, серийник, VID/PID)
unsafe fn extract_disk_info(
    device_info_set: HDEVINFO,
    dev_info: &SP_DEVINFO_DATA,
) -> Option<UsbDevice> {
    unsafe {
        // Буфер под Friendly Name (UTF-16, поэтому u16, а не u8)
        let mut name_buffer = [0u16; 256];
        let mut required = 0u32;

        // Приводим к срезу байтов
        let name_bytes = std::slice::from_raw_parts_mut(
            name_buffer.as_mut_ptr() as *mut u8,
            name_buffer.len() * 2,
        );

        let _ = SetupDiGetDeviceRegistryPropertyW(
            device_info_set,
            dev_info,
            SPDRP_FRIENDLYNAME,
            None,
            Some(name_bytes),
            Some(&mut required),
        );

        let friendly_name =
            String::from_utf16_lossy(&name_buffer[..(required as usize / 2).saturating_sub(1)]);

        // Получаем Instance ID для извлечения серийного номера
        let mut id_buffer = [0u16; 512];
        let mut id_required = 0u32;

        let _ = SetupDiGetDeviceInstanceIdW(
            device_info_set,
            dev_info,
            Some(&mut id_buffer),
            Some(&mut id_required),
        );

        let instance_id =
            String::from_utf16_lossy(&id_buffer[..(id_required as usize).saturating_sub(1)]);

        // Извлекаем серийный номер и VID/PID из Instance ID
        let serial = extract_serial(&instance_id);
        let (vid, pid) = extract_vid_pid(&instance_id);

        Some(UsbDevice {
            manufacturer: friendly_name,
            serial,
            filesystem: None,
            capacity: None,
            vid,
            pid,
        })
    }
}

/// Извлекает серийный номер из Instance ID диска
fn extract_serial(instance_id: &str) -> String {
    // USBSTOR\DISK&VEN_SANDISK&PROD_ULTRA&REV_1.00\1234567890AB&0
    // или USB\VID_0781&PID_5583\1234567890AB
    instance_id
        .split('\\')
        .last()
        .unwrap_or("")
        .split('&')
        .next()
        .unwrap_or("")
        .to_string()
}

/// Извлекает VID/PID из Instance ID (если есть)
fn extract_vid_pid(instance_id: &str) -> (Option<u16>, Option<u16>) {
    // USBSTOR\DISK&VEN_SANDISK&PROD_ULTRA&REV_1.00\...
    // Родительский USB ID можно получить через CM_Get_Parent, но здесь
    // мы парсим то, что есть в самом instance_id

    let mut vid = None;
    let mut pid = None;

    // Если instance_id начинается с USB\VID_xxxx&PID_xxxx
    if let Some(usb_part) = instance_id.strip_prefix("USB\\") {
        let parts: Vec<&str> = usb_part.split('\\').collect();
        if !parts.is_empty() {
            let id_part = parts[0];
            for segment in id_part.split('&') {
                if let Some(v) = segment.strip_prefix("VID_") {
                    vid = u16::from_str_radix(v, 16).ok();
                }
                if let Some(p) = segment.strip_prefix("PID_") {
                    pid = u16::from_str_radix(p, 16).ok();
                }
            }
        }
    }

    (vid, pid)
}
