<script lang="ts">
  import type { UsbFlashDevice, FormData } from "../types";

  let {
    device,
    saveDevice,
  }: {
    device?: UsbFlashDevice;
    saveDevice: (payload: FormData & { id?: string }) => void;
  } = $props();

  function createForm(device?: UsbFlashDevice): FormData {
    return {
      manufacturer: device?.manufacturer ?? "",
      serial: device?.serial ?? "",
      assigned_number: device?.assigned_number ?? "",
      capacity: device?.capacity ?? "",

      secret: device?.secret ?? false,
      special: device?.special ?? false,

      secclass: device?.secclass ?? "",
      max_secclass: device?.max_secclass ?? "",

      owner: device?.owner ?? "",
      register_number: String(device?.register_number ?? ""),
      conclusion_number: String(device?.conclusion_number ?? ""),
      prescription: String(device?.prescription ?? ""),
      zones: device?.zones ?? "",
    };
  }

  let form = $state(createForm());

  // let form = $state<FormData>({
  //   manufacturer: device?.manufacturer ?? "",
  //   serial: device?.serial ?? "",
  //   assigned_number: device?.assigned_number ?? "",
  //   capacity: device?.capacity ?? "",
  //
  //   secret: device?.secret ?? false,
  //   special: device?.special ?? false,
  //
  //   secclass: device?.secclass ?? "",
  //   max_secclass: device?.max_secclass ?? "",
  //
  //   owner: device?.owner ?? "",
  //   register_number: String(device?.register_number ?? ""),
  //   conclusion_number: String(device?.conclusion_number ?? ""),
  //   prescription: String(device?.prescription ?? ""),
  //   zones: device?.zones ?? "",
  // });
  //
  const isEdit = $derived(device !== undefined);

  function submit() {
    saveDevice({
      id: device?.id,
      ...form,
    });
  }
</script>

<form
  class="space-y-6"
  onsubmit={(e) => {
    e.preventDefault();
    submit();
  }}
>
  <div>
    <h3 class="text-lg font-semibold">Основная информация</h3>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mt-3">
      <label class="form-control">
        <div class="label">
          <span class="label-text">Производитель</span>
        </div>

        <input
          bind:value={form.manufacturer}
          class="input"
          placeholder="Kingston"
          required
        />
      </label>

      <label class="form-control">
        <div class="label">
          <span class="label-text">Серийный номер</span>
        </div>
        <input
          bind:value={form.serial}
          class="input"
          placeholder="60A44C3F91B2"
        />
      </label>

      <label class="form-control">
        <div class="label">
          <span class="label-text">Емкость</span>
        </div>

        <input bind:value={form.capacity} class="input" placeholder="64 GB" />
      </label>

      <label class="form-control">
        <div class="label">
          <span class="label-text"> Кому выдан / где используется </span>
        </div>

        <input bind:value={form.owner} class="input" />
      </label>

      <label class="form-control">
        <div class="label">
          <span class="label-text"> Рег. № (по журналу МНИ) </span>
        </div>

        <input bind:value={form.register_number} class="input" />
      </label>
    </div>
  </div>

  <div>
    <div class="mt-3">
      <label class="label cursor-pointer justify-start gap-3">
        <input bind:checked={form.secret} type="checkbox" class="checkbox" />
        <span>Для ГТ</span>
      </label>
    </div>
  </div>

  {#if form.secret}
    <div>
      <h3 class="text-lg font-semibold">Регистрационные данные</h3>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mt-3">
        <label class="form-control">
          <div class="label">
            <span class="label-text"> Зав. № / присвоенный при СП </span>
          </div>

          <input bind:value={form.assigned_number} class="input" />
        </label>

        <label class="form-control">
          <div class="label">
            <span class="label-text"> Предписание (Рег. №) </span>
          </div>

          <input bind:value={form.prescription} class="input" />
        </label>

        <label class="form-control">
          <div class="label">
            <span class="label-text"> Заключение о СП (Рег. №) </span>
          </div>

          <input bind:value={form.conclusion_number} class="input" />
        </label>

        <label class="form-control">
          <div class="label">
            <span class="label-text">Зоны</span>
          </div>

          <input bind:value={form.zones} class="input" placeholder="3/1,2/1" />
        </label>

        <label class="form-control">
          <div class="label">
            <span class="label-text">Гриф</span>
          </div>

          <select bind:value={form.secclass} class="select select-bordered">
            <option value="">Не указан</option>
            <option value="С">С</option>
            <option value="СС">СС</option>
            <option value="ОВ">ОВ</option>
            <option value="Н/С">Н/С</option>
          </select>
        </label>

        <label class="form-control">
          <div class="label">
            <span class="label-text"> Максимальный гриф </span>
          </div>

          <select bind:value={form.max_secclass} class="select select-bordered">
            <option value="">Не указан</option>
            <option value="С">С</option>
            <option value="СС">СС</option>
            <option value="ОВ">ОВ</option>
            <option value="Н/С">Н/С</option>
          </select>
        </label>

        <label class="label cursor-pointer justify-start gap-3 md:col-span-2">
          <input bind:checked={form.special} type="checkbox" class="checkbox" />
          <span>Специальное делопроизводство (СД)</span>
        </label>
      </div>
    </div>
  {/if}

  <div class="modal-action">
    <button type="submit" class="btn btn-success">
      {isEdit ? "Обновить" : "Добавить"}
    </button>
  </div>
</form>
