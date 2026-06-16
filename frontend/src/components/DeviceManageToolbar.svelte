<script lang="ts">
  let {
    search = $bindable(),
    onlyRegistered = $bindable(),
    onlySecret = $bindable(),
    onlyInternet = $bindable(),
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
      bind:checked={onlyRegistered}
      type="checkbox"
      class="checkbox checkbox-sm"
    />
    <span>Зарегистрированные</span>
  </label>

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

  <div class="ml-auto">
    <button class="btn btn-success btn-sm" onclick={onAdd}> Добавить </button>

    <button class="btn btn-primary btn-sm" disabled={selectedCount === 0}>
      Экспорт
    </button>
  </div>
</div>
