use crate::models::device::{UsbDevice};

pub async fn get_history_usb() -> Result<Vec<UsbDevice>, String> {
    let result = vec![UsbDevice::default(); 5];
    Ok(result)
}
