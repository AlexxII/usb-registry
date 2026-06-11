export type UsbFlashDevice = {
  id: string;
  label: string;
  manufacturer: string;
  serial: string;
  assigned_number: string, // заводсой номер, присвоенный при СП номер
  filesystem: string;
  capacity: string;
  registered: boolean;  // наличие в БД
  secret: boolean;      // для ГТ
  secclass: "С" | "СС" | "ОВ" | "Н/С" | null;
  maxsecclass: "С" | "СС" | "ОВ" | "Н/С" | null;
  special: boolean;     // СД
  owner: string | null;
  register_number: string | number | null;
  conclusion_number: string | number | null;
  prescription: string | number | null;
  zones: string | null;
  last_seen: string;
  first_seen: string;
};
