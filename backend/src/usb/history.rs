use super::utils::map_devices;
use crate::db::devices::get_devices;
use crate::errors::AppError;
use crate::models::device::{MappedDevice, UsbDevice};
use sqlx::SqlitePool;
use std::fs::File;
use std::thread;
use std::time::Duration;

use winreg::RegKey;
use winreg::enums::*;

pub async fn get_history_usb() -> Result<Vec<UsbDevice>, AppError> {
    let file = File::open("./usb.csv")?;
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .from_reader(file);
    let mut result = vec![];
    for r in rdr.deserialize() {
        let record: UsbDevice = r?;
        result.push(record)
    }
    Ok(result)
}

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

pub async fn get_history_usb_mapped(pool: &SqlitePool) -> Result<Vec<MappedDevice>, AppError> {
    thread::sleep(Duration::from_millis(100));

    let connected_usb = get_history_usb_from_os().await?;
    let usb_in_db = get_devices(&pool).await?;

    let mapped_devices = map_devices(connected_usb, usb_in_db, true)?;

    Ok(mapped_devices)
}
