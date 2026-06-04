<script lang="ts">
  import UsbDeviceCard from "../components/UsbDeviceCard.svelte";
  import NoDevices from "../components/NoDevices.svelte";
  import { usbDevices as devices } from "../data";
  // let devices: any[] = [];
</script>

{#if devices.length > 0}
  <div class="grid grid-cols-1 xl:grid-cols-2 gap-4">
    {#each devices as device}
      <UsbDeviceCard {device} />
    {/each}
  </div>
{:else}
  <div class="w-full max-w-xl mx-auto py-12">
    <NoDevices />
  </div>
{/if}
