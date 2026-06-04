<script lang="ts">
  import UsbSvg from "../assets/usbSvg.svelte";
</script>

<div class="hero min-h-[500px] rounded-box border border-base-300 bg-base-200">
  <div class="hero-content text-center">
    <div class="max-w-md">
      <div class="mb-6 flex justify-center">
        <div class="avatar placeholder">
          <div class="bg-base-300 text-base-content rounded-full w-24">
            <UsbSvg size="120"/>
          </div>
        </div>
      </div>

      <h1 class="text-3xl font-bold">Устройства не подключены</h1>

      <p class="py-4 text-base-content/70">
        В настоящий момент USB-накопители не обнаружены.
      </p>
    </div>
  </div>
</div>
