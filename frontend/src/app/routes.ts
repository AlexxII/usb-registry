import HomePage from "../pages/HomePage.svelte";
import DevicesPage from "../pages/DevicesPage.svelte";

export const routes = [
  {
    path: "/",
    component: HomePage,
  },
  {
    path: "/devices", 
    component: DevicesPage,
  }
];
