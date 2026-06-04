export type UsbFlashDevice = {
  id: string;
  label: string;
  manufacturer: string;
  serial: string;
  filesystem: string;
  capacity: string;
  registered: boolean;  // наличие в БД
  secret: boolean;      // для ГТ
  secclass: "С" | "СС" | "ОВ" | "Н/С" | null;
  special: boolean;     // СД
  owner: string | null;
  registerNumber: string | number | null;
  prescription: string | number | null;
  zones: string | null;
  lastSeen: string;
  firstSeen: string;
};
