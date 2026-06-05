<script lang="ts">
  import { usbDevices } from "../data";
  import UsbDeviceCard from "../components/UsbDeviceCard.svelte";

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
  <h1 class="text-3xl font-bold">История подключений</h1>
  <div class="overflow-x-auto">
    <table class="table table-zebra">
      <thead>
        <tr>
          <th>№ п.п.</th>
          <th>Производитель</th>
          <th>Гриф</th>
          <th>Ответственный</th>
          <th>Рег №</th>
          <th>s/n</th>
          <th>Предписание</th>
          <th>Зона 2</th>
          <th>СД</th>
          <th>Прочее</th>
        </tr>
      </thead>

      <tbody>
        {#each usbDevices as usb, index}
          <tr>
            <td>{index + 1}</td>
            <td>{usb.manufacturer}</td>
            <td>
              {#if usb.registered && !usb.secret}
                <div class="tooltip" data-tip="Для ИНТЕРНЕТа">
                  <div class="badge badge-sm bg-red-700">И</div>
                </div>
              {:else}
                {usb.secclass ?? "-"}
              {/if}
            </td>
            <td>{usb.owner ?? "-"}</td>
            <td>{usb.registerNumber ?? "-"}</td>
            <td>{usb.serial ?? "-"}</td>
            <td>{usb.prescription ?? "-"}</td>
            <td>{usb.zones ?? "-"}</td>
            <td>
              {#if usb.registered && usb.special}
                <div
                  aria-label="status"
                  class="status status-xl bg-red-700"
                ></div>
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
