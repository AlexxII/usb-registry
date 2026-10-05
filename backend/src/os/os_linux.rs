use std::fs::File;
use crate::errors::AppError;
use crate::models::device::UsbDevice;

// для подкючнных в настоящий момент
pub async fn get_current_usb_flash_drives() -> Result<Vec<UsbDevice>, AppError> {
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

// для истории подключений
pub async fn get_history_usb_from_os() -> Result<Vec<UsbDevice>, AppError> {
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
