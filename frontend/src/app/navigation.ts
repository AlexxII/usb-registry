import { UsbIcon, House, CalendarCheck2Icon } from "@lucide/svelte";
export const navigation = [
  {
    label: "Connected",
    href: "/",
    icon: House
  },
  {
    label: "History",
    href: "/history",
    icon: CalendarCheck2Icon
  },
  {
    label: "Management",
    href: "/device-manage",
    icon: UsbIcon
  }
];
