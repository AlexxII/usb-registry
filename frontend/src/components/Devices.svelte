<script lang="ts">
  import { usbDevices } from "../data";
</script>

<div class="overflow-x-auto">
  <table class="table table-sm">
    <!-- head -->
    <thead>
      <tr>
        <th>
          <label>
            <input type="checkbox" class="checkbox" />
          </label>
        </th>
        <th>Производитель</th>
        <th>s/n</th>
        <th>Номер при СП</th>
        <th>Емкость</th>
        <th>Рег. номер</th>
        <th>Владелец</th>
        <th>Гриф</th>
        <th>Макс.гриф</th>
        <th>Предписание</th>
        <th>Зоны 2</th>
      </tr>
    </thead>
    <tbody>
      {#each usbDevices as usb}
        <tr>
          <th>
            <label>
              <input type="checkbox" class="checkbox" />
            </label>
          </th>
          <td>{usb.manufacturer}</td>
          <td>{usb.serial}</td>
          <td>{usb.spnumber}</td>
          <th>{usb.capacity}</th>
          <th>{usb.registerNumber}</th>
          <th>{usb.owner}</th>
          <th>{usb.secclass}</th>
          <th>{usb.maxsecclass}</th>
          <th>{usb.prescription}</th>
          <th>{usb.zones}</th>
        </tr>
      {/each}
    </tbody>
  </table>
</div>
