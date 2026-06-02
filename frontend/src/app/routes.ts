import Connected from "../pages/CurrentConnectedPage.svelte";
import DevicesPage from "../pages/DevicesManagePage.svelte";
import History from "../pages/HistoryPage.svelte"

export const routes = [
  {
    path: "/",
    component: Connected,
  },
  {
    path: "/history", 
    component: History,
  },
  {
    path: "/device-manage",
    component: DevicesPage
  }
];
