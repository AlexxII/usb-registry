export type UsbFlashDevice = {
  id: string;
  label: string;
  manufacturer: string;
  serial: string;
  assignedNumber: string, // заводсой номер, присвоенный при СП номер
  filesystem: string;
  capacity: string;
  registered: boolean;  // наличие в БД
  secret: boolean;      // для ГТ
  secclass: "С" | "СС" | "ОВ" | "Н/С" | null;
  maxsecclass: "С" | "СС" | "ОВ" | "Н/С" | null;
  special: boolean;     // СД
  owner: string | null;
  registerNumber: string | number | null;
  conclusionNumber: string | number | null;
  prescription: string | number | null;
  zones: string | null;
  lastSeen: string;
  firstSeen: string;
};
