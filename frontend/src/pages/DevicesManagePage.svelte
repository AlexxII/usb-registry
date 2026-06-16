<script lang="ts">
  import { onMount } from "svelte";
  import AdminGuard from "../components/AdminGuard.svelte";
  import DeviceManageTable from "../components/DeviceManageTable.svelte";
  import DeviceManageToolbar from "../components/DeviceManageToolbar.svelte";
  import UsbDeviceForm from "../components/UsbDeviceForm.svelte";
  import { createDevice, getDevices } from "../api/devices";
  import type { UsbFlashDevice } from "../types";

  let usbDevices = $state<UsbFlashDevice[]>([]);
  let editedDevice = $state<UsbFlashDevice | undefined>();
  let createCounter = $state(0);
  let search = $state("");

  let onlyRegistered = $state(false);
  let onlySecret = $state(false);
  let onlyInternet = $state(false);

  let selected = $state<Set<string>>(new Set());

  let sortField = $state<keyof UsbFlashDevice>("manufacturer");
  let sortDirection = $state<"asc" | "desc">("asc");
  let modalRef = $state<HTMLDialogElement | null>(null);

  onMount(async () => {
    usbDevices = await getDevices();
  });

  function handleKeyDown(event: KeyboardEvent) {
    if (event.key === "Escape") {
      selected = new Set();
    }
  }

  function toggleSort(field: keyof UsbFlashDevice) {
    if (sortField === field) {
      sortDirection = sortDirection === "asc" ? "desc" : "asc";

      return;
    }

    sortField = field;
    sortDirection = "asc";
  }

  const filteredDevices = $derived.by(() => {
    let result = [...usbDevices];

    if (search.trim()) {
      const q = search.toLowerCase();

      result = result.filter((device) =>
        Object.values(device).some((value) =>
          String(value ?? "")
            .toLowerCase()
            .includes(q),
        ),
      );
    }

    if (onlyRegistered) {
      result = result.filter((d) => d.registered);
    }

    if (onlySecret) {
      result = result.filter((d) => d.secret);
    }

    if (onlyInternet) {
      result = result.filter((d) => d.registered && !d.secret);
    }

    result.sort((a, b) => {
      const av = String(a[sortField] ?? "");

      const bv = String(b[sortField] ?? "");

      return sortDirection === "asc"
        ? av.localeCompare(bv)
        : bv.localeCompare(av);
    });

    return result;
  });

  const allSelected = $derived.by(() => {
    if (!filteredDevices.length) {
      return false;
    }
    return filteredDevices.every((d) => selected.has(d.id));
  });

  function toggleAll() {
    if (allSelected) {
      selected = new Set();
      return;
    }
    selected = new Set(filteredDevices.map((d) => d.id));
  }

  function toggleDevice(id: string) {
    const next = new Set(selected);

    if (next.has(id)) {
      next.delete(id);
    } else {
      next.add(id);
    }

    selected = next;
  }

  function addNewDevice() {
    editedDevice = undefined;
    createCounter++;
    modalRef?.showModal();
  }

  function editDevice(device: UsbFlashDevice) {
    editedDevice = device;
    modalRef?.showModal();
  }

  function prepareData(data: UsbFlashDevice) {
    let { id, ...payload } = data;

    if (!payload.secret) {
      payload = {
        ...payload,
        secclass: null,
        max_secclass: null,
        special: false,
        assigned_number: "",
        conclusion_number: null,
        prescription: null,
        zones: null,
      };
    }

    return {
      ...payload,
      registered: true,
      destroyed: false,
    };
  }

  async function saveDevice(payload: UsbFlashDevice) {
    let data = prepareData(payload);
    let response = await createDevice(data);
    console.log(response);
  }
</script>

<svelte:window onkeydown={handleKeyDown} />

<dialog bind:this={modalRef} class="modal">
  <div class="modal-box max-w-2xl">
    {#key editedDevice?.id ?? `new-${createCounter}`}
      <UsbDeviceForm device={editedDevice} {saveDevice} />
    {/key}
  </div>

  <form method="dialog" class="modal-backdrop">
    <button>close</button>
  </form>
</dialog>

<div class="space-y-4">
  <AdminGuard>
    <DeviceManageToolbar
      bind:search
      bind:onlyRegistered
      bind:onlySecret
      bind:onlyInternet
      selectedCount={selected.size}
      onAdd={addNewDevice}
    />

    <DeviceManageTable
      devices={filteredDevices}
      {selected}
      {allSelected}
      {toggleAll}
      {toggleDevice}
      {toggleSort}
      {editDevice}
    />
  </AdminGuard>
</div>
