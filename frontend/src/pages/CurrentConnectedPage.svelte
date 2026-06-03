<script lang="ts">
  import UsbDeviceCard from "../components/UsbDeviceCard.svelte";
  import { usbDevices as devices } from "../data";


</script>

<div class="grid grid-cols-1 xl:grid-cols-2 gap-4">
  {#each devices as device}
    <UsbDeviceCard {device} />
  {/each}
</div>
