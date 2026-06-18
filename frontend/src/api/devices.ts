import type { UsbFlashDevice } from "../types";

const URL = "http://127.0.0.1:5151/usb/devices";


export async function getDevices(): Promise<UsbFlashDevice[]> {
  const response = await fetch(URL);

  if (!response.ok) {
    throw new Error(`HTTP ${response.status}`);
  }

  return response.json();
}

// export async function createDevice(device: UsbFlashDevice) {
export async function createDevice(device: any) {
  const response = await fetch(URL, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(device),
  });

  if (!response.ok) {
    throw new Error(`HTTP ${response.status}`);
  }

  return true;
}

export async function updateDevice(id: number, device: any) {
  const response = await fetch(`${URL}/${id}`, {
    method: "PUT",
    headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify(device)
  });

  if (!response.ok) {
    throw new Error(`HTTP ${response.status}`);
  }
  return true;
}
