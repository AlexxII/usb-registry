<script lang="ts">
  import { usbDevices } from "../data";
</script>

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
            <td> <button class="btn btn-ghost btn-xs">Подробнее</button></td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</div>
