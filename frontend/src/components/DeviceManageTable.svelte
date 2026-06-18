<script lang="ts">
  import { SquarePen, Trash2, CircleX } from "@lucide/svelte";
  import type { UsbFlashDevice } from "../types";

  let {
    devices,
    selected,
    allSelected,

    toggleAll,
    toggleDevice,

    toggleSort,
    editDevice,
    destroy,
    deleteDevice,
  }: {
    devices: UsbFlashDevice[];
    selected: Set<number>;
    allSelected: boolean;

    toggleAll: () => void;
    toggleDevice: (id: number) => void;

    toggleSort: (field: keyof UsbFlashDevice) => void;
    editDevice: (device: UsbFlashDevice) => void;
    destroy: (id: number) => void;
    deleteDevice: (id: number) => void;
  } = $props();
</script>

<div class="space-y-4">
  <!-- filters -->

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
              onclick={() => toggleSort("register_number")}
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
        {#each devices as usb}
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
            <td class="text-center">{usb.assigned_number}</td>
            <td class="text-center">{usb.register_number ?? "-"}</td>
            <td class="text-center">{usb.conclusion_number ?? "-"}</td>
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

            <td class="text-center">{usb.max_secclass ?? "-"}</td>
            <td class="text-center">{usb.zones ?? "-"}</td>
            <td class="text-center flex gap-1">
              <div class="tooltip tooltip-left" data-tip="Изменить">
                <SquarePen
                  class="cursor-pointer"
                  onclick={() => editDevice(usb)}
                />
              </div>
              <div class="tooltip tooltip-left" data-tip="Пометить как уничтоженное">
                <Trash2
                  class="cursor-pointer"
                  onclick={() => destroy(usb.id)}
                />
              </div>
              <div class="tooltip tooltip-left" data-tip="Удалить из базы">
                <CircleX
                  class="cursor-pointer"
                  onclick={() => deleteDevice(usb.id)}
                />
              </div>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</div>
