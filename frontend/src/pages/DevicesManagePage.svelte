<script lang="ts">
  import { onMount } from "svelte";
  import AdminGuard from "../components/AdminGuard.svelte";
  import DeviceManageTable from "../components/DeviceManageTable.svelte";
  import DeviceManageToolbar from "../components/DeviceManageToolbar.svelte";
  import UsbDeviceForm from "../components/UsbDeviceForm.svelte";
  import {
    createDevice,
    destroyDevice,
    undestroyDevice,
    getAllDevices,
    removeDevice,
    updateDevice,
    deleteDeviceCompletely,
  } from "../api/devices";
  import type { UsbFlashDevice } from "../types";
  import UsbDeviceImport from "../components/UsbDeviceImport.svelte";

  let usbDevices = $state<UsbFlashDevice[]>([]);
  let editedDevice = $state<UsbFlashDevice | undefined>();
  let createCounter = $state(0);
  let search = $state("");

  let onlyRegistered = $state(false);
  let onlySecret = $state(false);
  let onlyInternet = $state(false);
  let onlyDeleted = $state(false);
  let completeDeletion = $state(false);
  let onlyActive = $state(false);

  let selected = $state<Set<number>>(new Set());

  let sortField = $state<keyof UsbFlashDevice>("manufacturer");
  let sortDirection = $state<"asc" | "desc">("asc");
  let modalRef = $state<HTMLDialogElement | null>(null);
  let importModalRef = $state<HTMLDialogElement | null>(null);
  let confirmModalRef = $state<HTMLDialogElement | null>(null);
  let deletedId = $state<number | null>(null);

  onMount(async () => {
    usbDevices = await getAllDevices();
  });

  async function reloadDevices() {
    usbDevices = await getAllDevices();
  }

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

    if (onlyActive) {
      result = result.filter((d) => !d.destroyed);
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

    if (onlyDeleted) {
      result = result.filter((d) => d.deleted);
    } else {
      result = result.filter((d) => !d.deleted);
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

  function toggleDevice(id: number) {
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

  function importDevices() {
    importModalRef?.showModal();
  }

  // экспортировать сведения в csv
  function exportDevices() {
    console.log(selected);
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
    };
  }

  async function newDevice(payload: any) {
    try {
      let data = prepareData(payload);
      await createDevice(data);
      modalRef?.close();
      await reloadDevices();
    } catch (error) {
      alert("Не удалось сохранить устройство");
    }
  }

  async function update(id: any, payload: any) {
    try {
      await updateDevice(id, payload);
      modalRef?.close();
      await reloadDevices();
    } catch (error) {
      alert("Не удалось обновить устройство");
    }
  }

  async function save(payload: UsbFlashDevice) {
    const data = prepareData(payload);
    if (payload.id) {
      await update(payload.id, data);
    } else {
      await newDevice(data);
    }
  }

  // пометить как уничтоженное
  async function destroy(id: number) {
    try {
      await destroyDevice(id);
      await reloadDevices();
    } catch (error) {
      alert("Не удалось пометить как уничтоженное");
    }
  }

  // снять пометку об уничтоженнии
  async function unmarkDestroy(id: number) {
    try {
      await undestroyDevice(id);
      await reloadDevices();
    } catch (error) {
      alert("Не удалось снять пометку об уничтоженнии");
    }
  }

  // восстановить запись
  async function undoDeletion(id: number) {
    try {
      await removeDevice(id, false);
      await reloadDevices();
    } catch (error) {
      alert("Не удалось восстановить запись из удаленных");
    }
  }

  function handleDialogClose() {
    editedDevice = undefined;
    deletedId = null;
    completeDeletion = false;
  }

  function handleImportModalClose() {}

  async function deleteDeviceEx(id: number) {
    completeDeletion = true;
    deletedId = id;
    confirmModalRef?.showModal();
  }

  async function deleteDevice(id: number) {
    deletedId = id;
    confirmModalRef?.showModal();
  }

  async function hardDeletion() {
    if (deletedId === null) return;
    try {
      await deleteDeviceCompletely(deletedId);
      await reloadDevices();
    } catch (error) {
      alert("Не удалось удалить");
    }
  }

  async function softDeletion() {
    if (deletedId === null) return;
    try {
      await removeDevice(deletedId, true);
      await reloadDevices();
    } catch (error) {
      alert("Не удалось удалить");
    }
  }

  // импорт устройств
  async function importData(payload) {
    console.log(payload);
  }
</script>

<svelte:window onkeydown={handleKeyDown} />

<dialog bind:this={confirmModalRef} class="modal" onclose={handleDialogClose}>
  <div class="modal-box">
    <p class="py-4">
      Удалить устройство из базы? {completeDeletion
        ? "Операцию нельзя будет отменить."
        : ""}
    </p>
    <div class="modal-action">
      <form method="dialog">
        <button class="btn btn-soft btn-info">Отмена</button>
        <button
          class="btn btn-soft btn-warning"
          onclick={completeDeletion ? hardDeletion : softDeletion}
          >Удалить</button
        >
      </form>
    </div>
  </div>
</dialog>

<dialog bind:this={modalRef} class="modal" onclose={handleDialogClose}>
  <div class="modal-box max-w-2xl">
    {#key editedDevice?.id ?? `new-${createCounter}`}
      <UsbDeviceForm device={editedDevice} {save} />
    {/key}
  </div>

  <form method="dialog" class="modal-backdrop">
    <button>close</button>
  </form>
</dialog>

<dialog
  bind:this={importModalRef}
  class="modal"
  onclose={handleImportModalClose}
>
  <div class="modal-box">
    <UsbDeviceImport {importData} />
  </div>

  <form method="dialog" class="modal-backdrop">
    <button>close</button>
  </form>
</dialog>

<div class="space-y-4">
  <AdminGuard>
    <DeviceManageToolbar
      bind:search
      bind:onlyActive
      bind:onlyRegistered
      bind:onlySecret
      bind:onlyInternet
      bind:onlyDeleted
      selectedCount={selected.size}
      onAdd={addNewDevice}
      {importDevices}
      {exportDevices}
    />

    <DeviceManageTable
      devices={filteredDevices}
      {selected}
      {onlyDeleted}
      {allSelected}
      {toggleAll}
      {toggleDevice}
      {toggleSort}
      {editDevice}
      {destroy}
      {deleteDevice}
      {undoDeletion}
      {deleteDeviceEx}
      {unmarkDestroy}
    />
  </AdminGuard>
</div>
