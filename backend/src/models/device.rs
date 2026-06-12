use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UsbDevice {
    id: String,
    label: String,
    manufacturer: String,
    serial: String,
    assigned_number: String,
    filesystem: String,
    capacity: String,

    registered: bool,
    secret: bool,
    special: bool,

    secclass: Option<String>,
    maxsecclass: Option<String>,

    owner: Option<String>,
    register_number: Option<u32>,
    prescription: Option<u32>,
    conclusion_number: Option<u32>,
    zones: Option<String>,

    first_seen: String,
    last_seen: String,
}
