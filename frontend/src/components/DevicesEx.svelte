<script lang="ts">
  import { usbDevices } from "../data";
  import type { UsbFlashDevice } from "../types";

  let search = $state("");

  let onlyRegistered = $state(false);
  let onlySecret = $state(false);

  let selected = $state<Set<string>>(new Set());

  let sortField = $state<keyof UsbFlashDevice>("manufacturer");
  let sortDirection = $state<"asc" | "desc">("asc");

  function toggleSort(field: keyof UsbFlashDevice) {
    if (sortField === field) {
      sortDirection = sortDirection === "asc" ? "desc" : "asc";
      return;
    }

    sortField = field;
    sortDirection = "asc";
  }

  const filteredDevices = $derived.by(() => {
    let result = [...usbDevices];

    if (search.trim()) {
      const q = search.toLowerCase();

      result = result.filter((device) =>
        Object.values(device).some((value) =>
          String(value ?? "")
            .toLowerCase()
            .includes(q),
        ),
      );
    }

    if (onlyRegistered) {
      result = result.filter((d) => d.registered);
    }

    if (onlySecret) {
      result = result.filter((d) => d.secret);
    }

    result.sort((a, b) => {
      const av = String(a[sortField] ?? "");
      const bv = String(b[sortField] ?? "");

      return sortDirection === "asc"
        ? av.localeCompare(bv)
        : bv.localeCompare(av);
    });

    return result;
  });

  const allSelected = $derived.by(() => {
    if (filteredDevices.length === 0) return false;

    return filteredDevices.every((d) => selected.has(d.id));
  });

  function toggleAll() {
    if (allSelected) {
      selected = new Set();
      return;
    }

    selected = new Set(filteredDevices.map((d) => d.id));
  }

  function toggleDevice(id: string) {
    const next = new Set(selected);

    if (next.has(id)) {
      next.delete(id);
    } else {
      next.add(id);
    }

    selected = next;
  }
</script>

<div class="space-y-4">
  <!-- filters -->

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

    <div class="ml-auto badge badge-outline">
      Выбрано: {selected.size}
    </div>

    <button class="btn btn-primary btn-sm" disabled={selected.size === 0}>
      Редактировать выбранные ({selected.size})
    </button>
  </div>

  <!-- table -->

  <div class="overflow-x-auto rounded-box border border-base-300">
    <table class="table table-zebra table-pin-rows">
      <thead>
        <tr>
          <th>
            <input
              checked={allSelected}
              onchange={toggleAll}
              type="checkbox"
              class="checkbox checkbox-sm"
            />
          </th>

          <th>
            <button
              class="btn btn-ghost btn-xs"
              onclick={() => toggleSort("manufacturer")}
            >
              Производитель
            </button>
          </th>

          <th>
            <button
              class="btn btn-ghost btn-xs"
              onclick={() => toggleSort("serial")}
            >
              s/n
            </button>
          </th>

          <th>Емкость</th>

          <th>
            <button
              class="btn btn-ghost btn-xs"
              onclick={() => toggleSort("registerNumber")}
            >
              Рег. номер
            </button>
          </th>

          <th>Владелец</th>
          <th>Гриф</th>
          <th>Предписание</th>
          <th>Зоны</th>
        </tr>
      </thead>

      <tbody>
        {#each filteredDevices as usb}
          <tr class:selected-row={selected.has(usb.id)}>
            <td>
              <input
                checked={selected.has(usb.id)}
                onchange={() => toggleDevice(usb.id)}
                type="checkbox"
                class="checkbox checkbox-sm"
              />
            </td>

            <td>{usb.manufacturer}</td>
            <td>{usb.serial}</td>
            <td>{usb.capacity}</td>
            <td>{usb.registerNumber ?? "-"}</td>
            <td>{usb.owner ?? "-"}</td>

            <td>
              {#if usb.secret}
                <span class="badge badge-info">
                  {usb.secclass}
                </span>
              {:else if usb.registered}
                <span class="badge badge-error"> ИНТЕРНЕТ </span>
              {:else}
                -
              {/if}
            </td>

            <td>{usb.prescription ?? "-"}</td>
            <td>{usb.zones ?? "-"}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</div>

<style>
  .selected-row {
    background-color: color-mix(in srgb, var(--color-primary) 12%, transparent);
  }
</style>
