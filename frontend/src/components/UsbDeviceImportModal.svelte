<script lang="ts">
  import UsbDeviceImport from "./UsbDeviceImport.svelte";
  let {
    importModalRef = $bindable(),
    hasReport = $bindable(),
    importReport = $bindable(),
    handleImportModalClose,
    importData,
  } = $props();
</script>

<dialog
  bind:this={importModalRef}
  class="modal modal-bottom sm:modal-middle"
  onclose={handleImportModalClose}
>
  <div class="modal-box max-w-2xl">
    <form method="dialog">
      <button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2"
        >✕</button
      >
    </form>
    <h3 class="font-bold text-lg mb-4">Импорт USB-устройств</h3>
    <UsbDeviceImport {importData} />
    {#if hasReport}
      <div
        class="card bg-base-200 shadow-inner mt-6 p-4 border border-base-300"
      >
        <div
          class="stats stats-vertical lg:stats-horizontal shadow bg-base-100 w-full mb-4"
        >
          <div class="stat">
            <div class="stat-title text-success">Успешно добавлено</div>
            <!-- Читаем значение из `$state` -->
            <div class="stat-value text-success text-2xl">
              {importReport.successCount}
            </div>
            <div class="stat-desc">строк сохранено в БД</div>
          </div>

          <div class="stat">
            <div class="stat-title text-error">Пропущено строк</div>
            <div class="stat-value text-error text-2xl">
              {importReport.errorCount}
            </div>
            <div class="stat-desc">содержали ошибки</div>
          </div>
        </div>

        {#if importReport.errorCount > 0}
          <div
            class="overflow-x-auto max-h-48 border border-base-300 rounded-lg bg-base-100"
          >
            <table class="table table-xs table-pin-rows table-zebra w-full">
              <thead>
                <tr>
                  <th class="w-20 bg-base-300">Строка</th>
                  <th class="bg-base-300">Описание ошибки</th>
                </tr>
              </thead>
              <tbody>
                {#each importReport.errors as err}
                  <tr class="hover">
                    <td class="font-mono text-error font-bold text-center"
                      >{err.line}</td
                    >
                    <td class="whitespace-pre-wrap">{err.message}</td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        {/if}
      </div>
    {/if}
  </div>

  <form method="dialog" class="modal-backdrop">
    <button>close</button>
  </form>
</dialog>
