<script lang="ts">
  let manufacturer = $state("");
  let serial = $state("");
  let capacity = $state("");

  let registered = $state(false);
  let secret = $state(false);
  let special = $state(false);

  let secclass = $state<"С" | "СС" | "ОВ" | "Н/С" | "">("");

  let owner = $state("");
  let registerNumber = $state("");
  let prescription = $state("");
  let zones = $state("");

  function save() {
    const device = {
      manufacturer,
      serial,
      capacity,
      registered,
      secret,
      special,
      secclass: secclass || null,
      owner: owner || null,
      registerNumber: registerNumber || null,
      prescription: prescription || null,
      zones: zones || null,
    };

    console.log(device);

    // dispatch / callback
  }
</script>

<form
  class="space-y-6"
  onsubmit={(e) => {
    e.preventDefault();
    save();
  }}
>
  <div>
    <h3 class="font-semibold text-lg">
      Основная информация
    </h3>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mt-3">
      <label class="form-control">
        <div class="label">
          <span class="label-text">Производитель</span>
        </div>

        <input
          bind:value={manufacturer}
          class="input input-bordered"
          placeholder="Kingston"
          required
        />
      </label>

      <label class="form-control">
        <div class="label">
          <span class="label-text">Серийный номер</span>
        </div>

        <input
          bind:value={serial}
          class="input input-bordered"
          placeholder="60A44C3F91B2"
          required
        />
      </label>

      <label class="form-control">
        <div class="label">
          <span class="label-text">Емкость</span>
        </div>

        <input
          bind:value={capacity}
          class="input input-bordered"
          placeholder="64 GB"
        />
      </label>
    </div>
  </div>

  <div>
    <h3 class="font-semibold text-lg">
      Статус
    </h3>

    <div class="flex flex-wrap gap-4 mt-3">
      <label class="label cursor-pointer gap-2">
        <input
          bind:checked={registered}
          type="checkbox"
          class="checkbox"
        />
        <span>Зарегистрирован</span>
      </label>

      <label class="label cursor-pointer gap-2">
        <input
          bind:checked={secret}
          type="checkbox"
          class="checkbox"
        />
        <span>Для ГТ</span>
      </label>

      <label class="label cursor-pointer gap-2">
        <input
          bind:checked={special}
          type="checkbox"
          class="checkbox"
        />
        <span>СД</span>
      </label>
    </div>
  </div>

  {#if registered}
    <div>
      <h3 class="font-semibold text-lg">
        Регистрационные данные
      </h3>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mt-3">
        <label class="form-control">
          <div class="label">
            <span class="label-text">Ответственный</span>
          </div>

          <input
            bind:value={owner}
            class="input input-bordered"
          />
        </label>

        <label class="form-control">
          <div class="label">
            <span class="label-text">Рег. номер</span>
          </div>

          <input
            bind:value={registerNumber}
            class="input input-bordered"
          />
        </label>

        <label class="form-control">
          <div class="label">
            <span class="label-text">Предписание</span>
          </div>

          <input
            bind:value={prescription}
            class="input input-bordered"
          />
        </label>

        <label class="form-control">
          <div class="label">
            <span class="label-text">Зоны</span>
          </div>

          <input
            bind:value={zones}
            class="input input-bordered"
            placeholder="3/1,2/1"
          />
        </label>

        <label class="form-control ">
          <div class="label">
            <span class="label-text">Гриф</span>
          </div>

          <select
            bind:value={secclass}
            class="select select-bordered"
          >
            <option value="">Не указан</option>
            <option value="С">С</option>
            <option value="СС">СС</option>
            <option value="ОВ">ОВ</option>
            <option value="Н/С">Н/С</option>
          </select>
        </label>

        <label class="form-control ">
          <div class="label">
            <span class="label-text">Максимальный Гриф</span>
          </div>

          <select
            bind:value={secclass}
            class="select select-bordered"
          >
            <option value="">Не указан</option>
            <option value="С">С</option>
            <option value="СС">СС</option>
            <option value="ОВ">ОВ</option>
            <option value="Н/С">Н/С</option>
          </select>
        </label>
      </div>
    </div>
  {/if}

  <div class="modal-action">
    <button type="submit" class="btn btn-primary">
      Сохранить
    </button>
  </div>
</form>
