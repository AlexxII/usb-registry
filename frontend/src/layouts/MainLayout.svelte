<script lang="ts">
  import { onMount } from "svelte";
  import { route } from "@mateothegreat/svelte5-router";
  import { navigation } from "../app/navigation";

  let { children } = $props();

  let currentPath = $state(window.location.pathname);
  const updatePath = () => {
    currentPath = window.location.pathname;
  };
  onMount(() => {
    window.addEventListener("popstate", updatePath);

    return () => {
      window.removeEventListener("popstate", updatePath);
    };
  });
</script>

<div class="drawer lg:drawer-open">
  <input id="sidebar" type="checkbox" class="drawer-toggle" />

  <div class="drawer-content flex flex-col">
    <div class="navbar bg-base-200 border-b border-base-300">
      <div class="flex-none lg:hidden">
        <label for="sidebar" class="btn btn-square btn-ghost"> ☰ </label>
      </div>

      <div class="flex-1 px-2">
        <span class="text-lg font-semibold"> PIB </span>
      </div>
    </div>

    <main class="p-4">
      {@render children()}
    </main>
  </div>

  <div class="drawer-side">
    <label for="sidebar" class="drawer-overlay"></label>

    <aside class="bg-base-200 min-h-full w-64 border-r border-base-300">
      <div class="p-4 text-xl font-bold">Navigation</div>

      <ul class="menu w-full">
        {#each navigation as item}
          <li>
            <a
              use:route
              class:menu-active={currentPath === item.href}
              href={item.href}
              onclick={updatePath}
            >
              {item.label}
            </a>
          </li>
        {/each}
      </ul>
    </aside>
  </div>
</div>
