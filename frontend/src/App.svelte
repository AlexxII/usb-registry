<script lang="ts">
  import { Router } from "@mateothegreat/svelte5-router";
  import { routes } from "./app/routes";
  import MainLayout from "./layouts/MainLayout.svelte";
</script>

<MainLayout>
  <Router {routes} />
</MainLayout>
