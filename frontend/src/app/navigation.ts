import { UsbIcon, House, CalendarCheck2Icon } from "@lucide/svelte";
export const navigation = [
  {
    label: "Connected",
    href: "/",
    icon: House,
    title: "Домой"
  },
  {
    label: "History",
    href: "/history",
    icon: CalendarCheck2Icon,
    title: "История подключений"
  },
  {
    label: "Management",
    href: "/device-manage",
    icon: UsbIcon,
    title: "Управление"
  }
];
