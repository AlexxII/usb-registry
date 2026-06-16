<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "@mateothegreat/svelte5-router";
  import { navigation } from "../app/navigation";

  const AXUM_SERVER = "http://127.0.0.1:5151";

  let { children } = $props();

  let pageTitle = $derived(
    navigation.find((item) => item.href === currentPath)?.title ?? "",
  );

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

  // админка
  let passwordModal: HTMLDialogElement | null = null;
  let password = $state("");

  async function openManagement() {
    passwordModal?.showModal();
  }

  async function loginAdmin() {
    const response = await fetch(`${AXUM_SERVER}/auth/admin`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        password,
      }),
    });

    if (!response.ok) {
      alert("Неверный пароль");
      return;
    }
    const data = await response.json();
    sessionStorage.setItem("admin-token", data.token);
    passwordModal?.close();
    password = "";
    updatePath();
    goto("/device-manage");
  }

  function navigate(item: any) {
    if (item.href === "/device-manage") {
      openManagement();
      return;
    }
    goto(item.href);
    updatePath();
  }
</script>

<dialog bind:this={passwordModal} class="modal">
  <div class="modal-box">
    <h3 class="font-bold text-lg">Вход администратора</h3>

    <input
      bind:value={password}
      type="password"
      class="input input-bordered w-full mt-4"
      placeholder="Введите пароль"
    />

    <div class="modal-action">
      <button class="btn btn-primary" onclick={loginAdmin}> Войти </button>

      <form method="dialog">
        <button class="btn">Отмена</button>
      </form>
    </div>
  </div>
</dialog>

<div class="min-h-screen bg-base-100">
  <header class="navbar bg-base-200 border-b border-base-300">
    <div class="flex-1 px-4">
      <h1 class="text-xl font-semibold">
        {pageTitle}
      </h1>
    </div>
    <div class="ml-auto flex gap-2 pr-2">
      {#each navigation as item}
        {@const Icon = item.icon}
        <div class="tooltip tooltip-left" data-tip={item.title}>
          <button
            class="btn btn-ghost btn-sm"
            class:btn-active={currentPath === item.href}
            onclick={() => navigate(item)}
          >
            <Icon />
          </button>
        </div>
      {/each}
    </div>
  </header>

  <main class="p-4">
    {@render children()}
  </main>
</div>
