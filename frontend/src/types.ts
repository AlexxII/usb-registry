export type UsbFlashDevice = {
  id: number;
  label: string;
  manufacturer: string;
  serial: string;
  assigned_number: string, // заводской номер, присвоенный при СП номер
  filesystem: string;
  capacity: string;
  registered: boolean;  // наличие в БД
  secret: boolean;      // для ГТ
  secclass: "С" | "СС" | "ОВ" | "Н/С" | null;
  max_secclass: "С" | "СС" | "ОВ" | "Н/С" | null;
  special: boolean;     // СД
  owner: string | null;
  register_number: string | number | null;
  conclusion_number: string | number | null;
  prescription: string | number | null;
  zones: string | null;
  last_seen: string;
  first_seen: string;
  destroyed: boolean,
  deleted: boolean
};

export type FormData = {
  manufacturer: string;
  serial: string;
  assigned_number: string;
  capacity: string;

  secret: boolean;
  special: boolean;

  secclass: "С" | "СС" | "ОВ" | "Н/С" | "";
  max_secclass: "С" | "СС" | "ОВ" | "Н/С" | "";

  owner: string;
  register_number: string;
  conclusion_number: string;
  prescription: string;
  zones: string;
  destroyed: boolean,
  deleted: boolean

};

