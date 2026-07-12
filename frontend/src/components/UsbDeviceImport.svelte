<script lang="ts">
  let {
    importData,
  }: {
    importData: (payload: any) => void;
  } = $props();

  let files: FileList | null = $state(null);

  async function handleSubmit(e: SubmitEvent) {
    e.preventDefault();
    if (!files || files.length === 0) {
      alert("Пожалуйста, выберите файл перед импортом.");
      return;
    }

    const file = files[0];
    // 10 MB limit
    if (file.size > 10 * 1024 * 1024) {
      alert("Файл слишком большой!");
      return;
    }
    const reader = new FileReader();

    reader.onload = (e) => {
      const textContent = e.target?.result;

      importData({
        fileName: file.name,
        content: textContent,
      });
    };

    reader.onerror = () => {
      alert("Ошибка при чтении файла.");
    };

    reader.readAsText(file);
  }
</script>

<div>
  <form id="uploadForm" onsubmit={handleSubmit}>
    <div>
      <h3 class="text-lg font-semibold">Импортировать данные</h3>
      <div>
        <fieldset class="fieldset">
          <legend class="fieldset-legend"
            >Добавьте файл с данными формата *.csv</legend
          >
          <input type="file" accept="csv" bind:files class="file-input" />
        </fieldset>
      </div>
      <div class="modal-action">
        <button type="submit" class="btn btn-success">Импорт</button>
      </div>
    </div>
  </form>
</div>
