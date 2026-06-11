<script lang="ts">
  let { children } = $props();

  const expire = sessionStorage.getItem("admin-expire");

  // const isAuthorized = expire !== null && Date.now() < Number(expire);
  const isAuthorized = true
</script>

{#if isAuthorized}
  {@render children()}
{:else}
  <div class="hero min-h-[400px]">
    <div class="hero-content text-center">
      <div>
        <h1 class="text-2xl font-bold">Доступ запрещён</h1>

        <p class="mt-4 text-base-content/70">
          Требуется авторизация администратора.
        </p>
      </div>
    </div>
  </div>
{/if}
