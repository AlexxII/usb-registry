<script lang="ts">
  import UsbDeviceCard from "../components/UsbDeviceCard.svelte";
  import { usbDevices as devices } from "../data.js";

  type UsbFlashDevice = {
    id: string;
    label: string;
    manufacturer: string;
    serial: string;
    filesystem: string;
    capacity: string;
    status: "mounted" | "blocked" | "readonly";
    department: string;
    registerNumber: string | number;
    prescription: string | number;
    zones: string;
    lastSeen: string;
    firstSeen: string;
  };

</script>

<div class="grid grid-cols-1 xl:grid-cols-2 gap-4">
  {#each devices as device}
    <UsbDeviceCard {device} />
  {/each}
</div>
