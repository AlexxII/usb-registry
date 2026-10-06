<script lang="ts">
  import { onMount } from "svelte";
  import UsbDeviceCard from "../components/UsbDeviceCard.svelte";
  import type { UsbFlashDevice } from "../types";
    import { getHistoryDevices } from "../api/devices";

  let devices = $state<UsbFlashDevice[]>([]);

  onMount(async () => {
    devices = await getHistoryDevices();
  });

  // 1. Создаем реактивное состояние для выбранного устройства
  let selectedDevice = $state<any>(null);
  let modalRef = $state<HTMLDialogElement | null>(null);

  function openDetails(device: any) {
    selectedDevice = device;
    modalRef?.showModal();
  }
</script>

<dialog bind:this={modalRef} id="my_modal_2" class="modal">
  <div class="modal-box max-w-2xl">
    {#if selectedDevice}
      <UsbDeviceCard device={selectedDevice} />
    {/if}
  </div>
  <form method="dialog" class="modal-backdrop">
    <button>close</button>
  </form>
</dialog>

<div class="space-y-4">
  <div class="overflow-x-auto">
    <table class="table table-zebra">
      <thead>
        <tr>
          <th class="text-center">№ п.п.</th>
          <th class="text-center">Производитель</th>
          <th class="text-center">Рег №</th>
          <th class="text-center">Гриф</th>
          <th class="text-center">s/n</th>
          <th class="text-center">Предписание</th>
          <th class="text-center">Зона 2</th>
          <th class="text-center">Ответственный</th>
          <th class="text-center">СД</th>
          <th class="text-center">Прочее</th>
        </tr>
      </thead>

      <tbody>
        {#each devices as usb, index}
          <tr class:destroyed={usb.destroyed}>
            <td class="text-center">{index + 1}</td>
            <td class="text-center">{usb.manufacturer}</td>
            <td class="text-center">{usb.register_number ?? "-"}</td>
            <td class="text-center">
              {#if usb.registered && !usb.secret}
                <div class="tooltip" data-tip="ИНТЕРНЕТ">
                  <div class="badge badge-sm bg-red-700">Интернет</div>
                </div>
              {:else}
                {usb.secclass ?? "-"}
              {/if}
            </td>
            <td class="text-center">{usb.serial ?? "-"}</td>
            <td class="text-center">{usb.prescription ?? "-"}</td>
            <td class="text-center">{usb.zones ?? "-"}</td>
            <td class="text-center">{usb.owner ?? "-"}</td>
            <td class="text-center">
              {#if usb.registered && usb.special}
                <div class="tooltip" data-tip="СПЕЦИАЛЬНОЕ ДЕЛОПРОИЗВОДСТВО">
                  <div
                    aria-label="status"
                    class="status status-xl bg-red-700"
                  ></div>
                </div>
              {:else}
                -
              {/if}
            </td>
            <td class="text-center">
              <button
                class="btn btn-ghost btn-xs"
                onclick={() => openDetails(usb)}>Подробнее</button
              ></td
            >
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</div>
