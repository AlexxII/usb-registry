import CompsPage from "../pages/CompsPage.svelte";
import DevicesPage from "../pages/DevicesPage.svelte";

export const routes = [
  {
    path: "/",
    component: DevicesPage,
  },
  {
    path: "/ovt", 
    component: CompsPage,
  }
];
