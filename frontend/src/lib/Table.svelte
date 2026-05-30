<script lang="ts">
  import { usbDevices } from "../data.js";
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
        <th>Модель</th>
        <th>Серийный номер</th>
        <th>VID</th>
        <th>PID</th>
        <th>Емкость</th>
        <th>Владелец</th>
        <th>Подразделение</th>
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
          <td>{usb.vendor}</td>
          <td>{usb.product}</td>
          <td>{usb.serial}</td>
          <th>{usb.vid}</th>
          <th>{usb.pid}</th>
          <th>{usb.capacityGb}</th>
          <th>{usb.owner}</th>
          <th>{usb.department}</th>
        </tr>
      {/each}
    </tbody>
  </table>
</div>
