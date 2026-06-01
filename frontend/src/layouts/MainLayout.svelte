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

<div class="min-h-screen bg-base-100">
  <header class="navbar bg-base-200 border-b border-base-300">
    <div class="ml-auto flex gap-2 pr-2">
      {#each navigation as item}
        {@const Icon = item.icon}
        <a
          use:route
          href={item.href}
          class="btn btn-ghost btn-sm"
          class:btn-active={currentPath === item.href}
          onclick={updatePath}
        >
          <Icon />
        </a>
      {/each}
    </div>
  </header>

  <main class="p-4">
    {@render children()}
  </main>
</div>
