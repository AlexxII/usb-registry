<script lang="ts">
  import { onMount } from "svelte";
  import UsbDeviceCard from "../components/UsbDeviceCard.svelte";
  import type { UsbFlashDevice } from "../types";

  let usbDevices = $state<UsbFlashDevice[]>([]);
  onMount(async () => {
    try {
      const response = await fetch("http://127.0.0.1:5151/usb");
      usbDevices = await response.json();
      console.log(usbDevices);
    } catch (error) {
      console.error("Ошибка загрузки USB устройств:", error);
    }
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
  <div class="flex justify-end">
    <button class="btn btn-info btn-sm">Обновить</button>
  </div>
  <div class="overflow-x-auto">
    <table class="table table-zebra">
      <thead>
        <tr>
          <th>№ п.п.</th>
          <th>Производитель</th>
          <th>Рег №</th>
          <th>Гриф</th>
          <th>s/n</th>
          <th>Предписание</th>
          <th>Зона 2</th>
          <th>Ответственный</th>
          <th>СД</th>
          <th>Прочее</th>
        </tr>
      </thead>

      <tbody>
        {#each usbDevices as usb, index}
          <tr>
            <td>{index + 1}</td>
            <td>{usb.manufacturer}</td>
            <td>{usb.registerNumber ?? "-"}</td>
            <td>
              {#if usb.registered && !usb.secret}
                <div class="tooltip" data-tip="ИНТЕРНЕТ">
                  <div class="badge badge-sm bg-red-700">Интернет</div>
                </div>
              {:else}
                {usb.secclass ?? "-"}
              {/if}
            </td>
            <td>{usb.serial ?? "-"}</td>
            <td>{usb.prescription ?? "-"}</td>
            <td>{usb.zones ?? "-"}</td>
            <td>{usb.owner ?? "-"}</td>
            <td>
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
            <td>
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
