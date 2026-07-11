<script lang="ts">
  import { FileDown, FileUp } from "@lucide/svelte";

  let {
    search = $bindable(),
    onlyRegistered = $bindable(),
    onlyActive = $bindable(),
    onlySecret = $bindable(),
    onlyInternet = $bindable(),
    onlyDeleted = $bindable(),
    importDevices,
    exportDevices,
    selectedCount,
    onAdd,
  } = $props();
</script>

<div class="flex flex-wrap gap-3">
  <label class="input w-80">
    <input bind:value={search} type="search" placeholder="Поиск..." />
  </label>

  <label class="label cursor-pointer gap-2">
    <input
      bind:checked={onlyActive}
      type="checkbox"
      class="checkbox checkbox-sm"
    />
    <span>Активные</span>
  </label>

  <!-- <label class="label cursor-pointer gap-2"> -->
  <!--   <input -->
  <!--     bind:checked={onlyRegistered} -->
  <!--     type="checkbox" -->
  <!--     class="checkbox checkbox-sm" -->
  <!--   /> -->
  <!--   <span>Зарегистрированные</span> -->
  <!-- </label> -->

  <label class="label cursor-pointer gap-2">
    <input
      bind:checked={onlySecret}
      type="checkbox"
      class="checkbox checkbox-sm"
    />
    <span>Секретные</span>
  </label>

  <label class="label cursor-pointer gap-2">
    <input
      bind:checked={onlyInternet}
      type="checkbox"
      class="checkbox checkbox-sm"
    />
    <span>Интернет</span>
  </label>

  <label class="label cursor-pointer gap-2">
    <input
      bind:checked={onlyDeleted}
      type="checkbox"
      class="checkbox checkbox-sm"
    />
    <span>Удаленные</span>
  </label>

  <div class="ml-auto gap-2">
    {#if !onlyDeleted}
      <button class="btn btn-success btn-sm" onclick={onAdd}> Добавить </button>
      <button class="btn btn-secondary btn-sm" onclick={importDevices}
        >Импорт</button
      >
    {/if}
    <button
      class="btn btn-secondary btn-sm"
      disabled={selectedCount === 0}
      onclick={exportDevices}>Экспорт</button
    >
  </div>
</div>
