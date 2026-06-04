<script lang="ts">
  import { usbDevices } from "../data";
</script>

<div class="space-y-4">
  <h1 class="text-3xl font-bold">История подключений</h1>
  <div class="overflow-x-auto">
    <table class="table table-zebra">
      <thead>
        <tr>
          <th>Производитель</th>
          <th>Ответственный</th>
          <th>Рег №</th>
          <th>s/n</th>
          <th>Предписание</th>
          <th>Зоны 2</th>
          <th>Назначение</th>
          <th>Прочее</th>
        </tr>
      </thead>

      <tbody>
        {#each usbDevices as usb}
          <tr>
          <td>{usb.manufacturer}</td>
          <td>{usb.owner}</td>
          <td>{usb.registerNumber}</td>
          <td>{usb.serial}</td>
          <td>{usb.prescription}</td>
          <td>{usb.zones}</td>
          <td>{usb.capacity}</td>
          <td>{usb.secclass}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</div>
