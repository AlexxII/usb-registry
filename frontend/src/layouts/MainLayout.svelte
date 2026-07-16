<script lang="ts">
  import { onMount, tick } from "svelte";
  import { goto } from "@mateothegreat/svelte5-router";
  import { navigation } from "../app/navigation";

  const AXUM_SERVER = "http://127.0.0.1:5151";

  let { children } = $props();

  let currentPath = $state(window.location.pathname);
  let isAuthorized = $state(false);

  let pageTitle = $derived(
    navigation.find((item) => item.href === currentPath)?.title ?? "",
  );

  const updatePath = () => {
    currentPath = window.location.pathname;
  };

  function hasLocalToken(): boolean {
    return !!sessionStorage.getItem("admin-token");
  }

  async function checkTokenValidity(): Promise<boolean> {
    const token = sessionStorage.getItem("admin-token");
    if (!token) return false;

    try {
      const response = await fetch(`${AXUM_SERVER}/auth/verify`, {
        method: "GET",
        headers: { Authorization: `Bearer ${token}` },
      });
      return response.ok;
    } catch {
      return false;
    }
  }

  async function logout() {
    sessionStorage.removeItem("admin-token");
    isAuthorized = false;
    if (currentPath === "/device-manage") {
      goto("/");
      await tick();
      updatePath();
    }
  }

  onMount(async () => {
    window.addEventListener("popstate", updatePath);
    const isValid = await checkTokenValidity();
    isAuthorized = isValid;

    if (currentPath === "/device-manage" && !isValid) {
      sessionStorage.removeItem("admin-token");
      goto("/");
      await tick();
      updatePath();
    }

    return () => {
      window.removeEventListener("popstate", updatePath);
    };
  });

  // Модалка входа
  let passwordModal: HTMLDialogElement | null = null;
  let password = $state("");

  async function loginAdmin() {
    const response = await fetch(`${AXUM_SERVER}/auth/admin`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ password }),
    });

    if (!response.ok) {
      alert("Неверный пароль");
      return;
    }
    const data = await response.json();
    sessionStorage.setItem("admin-token", data.token);

    isAuthorized = true;
    passwordModal?.close();
    password = "";
    goto("/device-manage");
    await tick();
    updatePath();
  }

  // Переменные для модалки смены пароля
  let changePasswordModal: HTMLDialogElement | null = null;
  let oldPassword = $state("");
  let newPassword = $state("");

  async function submitPasswordChange() {
    const token = sessionStorage.getItem("admin-token");

    const response = await fetch(`${AXUM_SERVER}/auth/change-password`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      },
      body: JSON.stringify({
        old_password: oldPassword,
        new_password: newPassword,
      }),
    });

    if (response.status === 401) {
      alert("Сессия истекла. Войдите заново.");
      logout();
      changePasswordModal?.close();
      return;
    }

    if (!response.ok) {
      alert("Неверный старый пароль");
      return;
    }

    alert("Пароль успешно изменен!");
    changePasswordModal?.close();
    oldPassword = "";
    newPassword = "";
  }

  async function navigate(item: any) {
    if (item.href === "/device-manage") {
      if (hasLocalToken() && (await checkTokenValidity())) {
        isAuthorized = true;
        goto("/device-manage");
        await tick();
        updatePath();
      } else {
        isAuthorized = false;
        sessionStorage.removeItem("admin-token");
        passwordModal?.showModal();
      }
      return;
    }
    goto(item.href);
    await tick();
    updatePath();
  }
</script>

<!-- Модалка Входа -->
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
      <form method="dialog"><button class="btn">Отмена</button></form>
    </div>
  </div>
</dialog>

<!-- Новая модалка Смены Пароля -->
<dialog bind:this={changePasswordModal} class="modal">
  <div class="modal-box">
    <h3 class="font-bold text-lg">Изменение пароля</h3>

    <input
      bind:value={oldPassword}
      type="password"
      class="input input-bordered w-full mt-4"
      placeholder="Старый пароль"
    />
    <input
      bind:value={newPassword}
      type="password"
      class="input input-bordered w-full mt-2"
      placeholder="Новый пароль"
    />

    <div class="modal-action">
      <button class="btn btn-warning" onclick={submitPasswordChange}>
        Сохранить
      </button>
      <form method="dialog"><button class="btn">Отмена</button></form>
    </div>
  </div>
</dialog>

<div class="min-h-screen bg-base-100">
  <header class="navbar bg-base-200 border-b border-base-300 px-4">
    <div class="flex-1">
      <h1 class="text-xl font-semibold">{pageTitle}</h1>
    </div>

    <div class="flex items-center gap-4">
      <div class="flex gap-2">
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

      <!-- Кнопки управления аккаунтом админа -->
      {#if isAuthorized}
        <div class="flex gap-2">
          <button
            class="btn btn-outline btn-warning btn-sm"
            onclick={() => changePasswordModal?.showModal()}
          >
            Сменить пароль
          </button>
          <button class="btn btn-outline btn-error btn-sm" onclick={logout}>
            Выйти
          </button>
        </div>
      {/if}
    </div>
  </header>

  <main class="p-4">
    {@render children()}
  </main>
</div>
