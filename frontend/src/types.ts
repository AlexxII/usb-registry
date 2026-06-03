export type UsbFlashDevice = {
  id: string;
  label: string;
  manufacturer: string;
  serial: string;
  filesystem: string;
  capacity: string;
  registered: boolean;  // наличие в БД
  secret: boolean;      // для ГТ
  special: boolean;     // СД
  owner: string | null;
  registerNumber: string | number;
  prescription: string | number;
  zones: string;
  lastSeen: string;
  firstSeen: string;
};
