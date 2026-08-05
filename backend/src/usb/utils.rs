use crate::errors::DeviceError;
use crate::models::device::{Device, MappedDevice, UsbDevice};
use std::collections::HashMap;

pub fn map_devices(
    usb_devices: Vec<UsbDevice>,
    usb_in_db: Vec<Device>,
    is_connected: bool,
) -> Result<Vec<MappedDevice>, DeviceError> {
    let mut db_map: HashMap<String, Device> = usb_in_db
        .into_iter()
        .map(|d| (d.serial.clone(), d))
        .collect();

    let result = usb_devices
        .into_iter()
        .map(|usb| {
            if let Some(mapped_usb) = db_map.remove(&usb.serial) {
                // Если устройство есть в БД, забираем его данные
                MappedDevice {
                    id: Some(mapped_usb.id),
                    manufacturer: Some(mapped_usb.manufacturer),
                    serial: Some(mapped_usb.serial),
                    filesystem: usb.filesystem,
                    capacity: usb.capacity,
                    registered: true,
                    assigned_number: Some(mapped_usb.assigned_number),
                    owner: mapped_usb.owner,
                    register_number: mapped_usb.register_number,
                    conclusion_number: mapped_usb.conclusion_number,
                    prescription: mapped_usb.prescription,
                    secret: mapped_usb.secret,
                    special: mapped_usb.special,
                    max_secclass: mapped_usb.max_secclass,
                    secclass: mapped_usb.secclass,
                    zones: mapped_usb.zones,
                    connected: is_connected,
                    destroyed: mapped_usb.destroyed,
                    deleted: mapped_usb.deleted,
                }
            } else {
                // Если устройства нет в БД
                MappedDevice {
                    id: None,
                    manufacturer: Some(usb.manufacturer),
                    serial: Some(usb.serial),
                    filesystem: usb.filesystem,
                    capacity: usb.capacity,
                    registered: false,
                    connected: is_connected,
                    ..Default::default()
                }
            }
        })
        .collect();
    Ok(result)
}
