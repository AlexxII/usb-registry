<script lang="ts">
  const { device } = $props();
</script>

<div class="card bg-base-200 border border-base-300 shadow-md w-full max-w-xl">
  <div class="card-body gap-5">
    <!-- header -->
    <div class="flex items-start justify-between">
      <div>
        <h2 class="card-title text-lg">
          {device.label}
        </h2>

        <p class="text-sm text-base-content/60">USB Flash Device</p>
      </div>

      {#if device.registered}
        <div class="tooltip" data-tip="Зарегистрирован">
          <div aria-label="status" class="status status-xl bg-green-700"></div>
        </div>
      {:else}
        <div class="tooltip" data-tip="Нет информации">
          <div aria-label="status" class="status status-xl bg-red-700"></div>
        </div>
      {/if}
    </div>

    <!-- system info -->
    <div>
      <div class="flex items-center gap-2 mb-3">
        <div class="badge badge-outline badge-sm">OS</div>

        <span class="text-sm text-base-content/60">
          Данные операционной системы
        </span>
      </div>

      <div class="grid grid-cols-2 gap-x-6 gap-y-3">
        <div>
          <p class="text-base-content/60 text-sm">Производитель</p>

          <p>{device.manufacturer}</p>
        </div>

        <div>
          <p class="text-base-content/60 text-sm">Файловая система</p>

          <p>{device.filesystem}</p>
        </div>

        <div>
          <p class="text-base-content/60 text-sm">Объем</p>

          <p>{device.capacity}</p>
        </div>

        <!-- serial -->
        <div class="col-span-2">
          <div
            class="rounded-box bg-base-300/50 border border-primary/30 px-3 py-2"
          >
            <p class="text-xs uppercase tracking-wide text-primary">
              Serial Number
            </p>

            <p class="font-mono text-sm break-all">
              {device.serial}
            </p>
          </div>
        </div>
      </div>
    </div>

    <!-- registry info -->
    {#if device.registered}
      <div class="rounded-box border border-secondary/20 bg-secondary/5 p-4">
        <div class="flex items-center gap-2 mb-4">
          <div class="badge badge-secondary badge-sm">Registry</div>

          <span class="text-sm text-base-content/60">
            Учетные данные устройства
          </span>
          {#if device.secret}
            <div class="tooltip" data-tip="Гриф секретности">
              <div class="badge badge-sm bg-blue-700">{device.secclass}</div>
            </div>
          {:else}
            <div class="tooltip" data-tip="Доложить в 13 группу (тел. 30-39)">
              <div class="badge badge-sm bg-red-700">ИНТЕРНЕТ</div>
            </div>
          {/if}
          {#if device.special}
            <div class="tooltip" data-tip="Специальное делопроизводство">
              <div class="badge badge-sm bg-red-700">СД</div>
            </div>
          {/if}
        </div>

        <div class="grid grid-cols-2 gap-x-6 gap-y-4">
          <div>
            <p class="text-base-content/60 text-sm">Ответственный</p>

            <p class="font-medium">
              {device.owner}
            </p>
          </div>

          <div>
            <p class="text-base-content/60 text-sm">Регистрационный №</p>

            <p class="font-medium">
              {device.registerNumber}
            </p>
          </div>

          <div>
            <p class="text-base-content/60 text-sm">Предписание</p>

            <p class="font-medium">
              {device.prescription}
            </p>
          </div>

          <div>
            <p class="text-base-content/60 text-sm">Зоны (м.)</p>

            <p class="font-medium">
              {device.zones}
            </p>
          </div>
        </div>
      </div>
    {/if}

    <!-- timestamps -->
    <div class="grid grid-cols-2 gap-6">
      <div>
        <p class="text-base-content/60 text-sm">Первое подключение</p>

        <p>{device.firstSeen}</p>
      </div>

      <div>
        <p class="text-base-content/60 text-sm">Последнее подключение</p>

        <p>{device.lastSeen}</p>
      </div>
    </div>
  </div>
</div>
