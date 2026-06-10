<script lang="ts">
  import { usbDevices } from "../data";
  import type { UsbFlashDevice } from "../types";
  import { SquarePen } from "@lucide/svelte";
  import UsbDeviceAddForm from "./UsbDeviceAddForm.svelte";

  let search = $state("");

  let onlyRegistered = $state(false);
  let onlySecret = $state(false);

  let selected = $state<Set<string>>(new Set());

  let sortField = $state<keyof UsbFlashDevice>("manufacturer");
  let sortDirection = $state<"asc" | "desc">("asc");

  function handleKeyDown(event: KeyboardEvent) {
    if (event.key === "Escape") {
      selected = new Set();
      return;
    }
  }

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

  function addNewDevice() {
    modalRef?.showModal();
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

  let modalRef = $state<HTMLDialogElement | null>(null);
</script>

<svelte:window onkeydown={handleKeyDown} />

<dialog bind:this={modalRef} id="my_modal_2" class="modal">
  <div class="modal-box max-w-2xl">
    <UsbDeviceAddForm />
  </div>
  <form method="dialog" class="modal-backdrop">
    <button>close</button>
  </form>
</dialog>

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

    <div class="ml-auto">
      <button class="btn btn-success btn-sm" onclick={addNewDevice}>
        Добавить
      </button>

      <button class="btn btn-primary btn-sm" disabled={selected.size === 0}>
        Экспорт
      </button>
    </div>
  </div>

  <!-- table -->

  <div class="overflow-x-auto rounded-box border border-base-300">
    <table class="table table-zebra table-pin-rows">
      <thead>
        <tr>
          <th class="text-center">
            <input
              checked={allSelected}
              onchange={toggleAll}
              type="checkbox"
              class="checkbox checkbox-sm"
            />
          </th>

          <th class="text-center">
            Производитель
            <br />
            Модель</th
          >
          <th class="text-center">Емкость</th>
          <th class="text-center">s/n</th>
          <th class="whitespace-normal text-center">
            Заводской номер
            <br />
            (№ при СП)
          </th>

          <th class="text-center">
            <button
              class="btn btn-ghost btn-xs"
              onclick={() => toggleSort("registerNumber")}
            >
              Рег. №
            </button>
          </th>

          <th class="text-center">
            Заключение
            <br />
            о СП
          </th>
          <th class="text-center">Предписание</th>
          <th class="text-center">
            Кому выдан,
            <br />
            где используется
          </th>
          <th class="text-center">Гриф</th>
          <th class="text-center">Макс.гриф</th>
          <th class="text-center">Зоны 2</th>
          <th class="text-center"></th>
        </tr>
      </thead>

      <tbody>
        {#each filteredDevices as usb}
          <tr class:selected-row={selected.has(usb.id)}>
            <td class="text-center">
              <input
                checked={selected.has(usb.id)}
                onchange={() => toggleDevice(usb.id)}
                type="checkbox"
                class="checkbox checkbox-sm"
              />
            </td>

            <td class="text-center">{usb.manufacturer}</td>
            <td class="text-center">{usb.capacity}</td>
            <td class="text-center">{usb.serial}</td>
            <td class="text-center">{usb.assignedNumber}</td>
            <td class="text-center">{usb.registerNumber ?? "-"}</td>
            <td class="text-center">{usb.conclusionNumber ?? "-"}</td>
            <td class="text-center">{usb.prescription ?? "-"}</td>
            <td class="text-center">{usb.owner ?? "-"}</td>

            <td class="text-center">
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

            <td class="text-center">{usb.maxsecclass ?? "-"}</td>
            <td class="text-center">{usb.zones ?? "-"}</td>
            <td class="text-center">
              <button class="btn btn-sm">
                <SquarePen />
              </button>
            </td>
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
