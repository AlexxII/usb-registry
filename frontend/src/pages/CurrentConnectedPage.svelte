<script lang="ts">
  type UsbFlashDevice = {
    id: string;
    label: string;
    manufacturer: string;
    serial: string;
    filesystem: string;
    capacity: string;
    status: "mounted" | "blocked" | "readonly";
    department: string;
    registerNumber: string | number;
    prescription: string | number;
    zones: string;
    lastSeen: string;
    firstSeen: string;
  };

  let device: UsbFlashDevice = {
    id: "usb-01",
    label: "KINGSTON_DT",
    manufacturer: "Kingston",
    serial: "60A44C3F91B2",
    filesystem: "exFAT",
    capacity: "64 GB",
    status: "mounted",
    department: "13 группа",
    registerNumber: 131,
    prescription: 3431,
    zones: "3/1,2/1",
    lastSeen: "2026-06-02 12:44:08",
    firstSeen: "2025-01-13 09:45:23",
  };

  const statusConfig = {
    mounted: {
      label: "Mounted",
      class: "badge-success",
    },
    blocked: {
      label: "Blocked",
      class: "badge-error",
    },
    readonly: {
      label: "Read Only",
      class: "badge-warning",
    },
  };
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

      <div class={`badge ${statusConfig[device.status].class}`}>
        {statusConfig[device.status].label}
      </div>
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
    <div class="rounded-box border border-secondary/20 bg-secondary/5 p-4">
      <div class="flex items-center gap-2 mb-4">
        <div class="badge badge-secondary badge-sm">Registry</div>

        <span class="text-sm text-base-content/60">
          Учетные данные устройства
        </span>
      </div>

      <div class="grid grid-cols-2 gap-x-6 gap-y-4">
        <div>
          <p class="text-base-content/60 text-sm">Подразделение</p>

          <p class="font-medium">
            {device.department}
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
          <p class="text-base-content/60 text-sm">Зоны</p>

          <p class="font-medium">
            {device.zones}
          </p>
        </div>
      </div>
    </div>

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
