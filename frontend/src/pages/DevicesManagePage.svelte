<script lang="ts">
  import DevicesEx from "../components/DevicesEx.svelte";
  import AdminGuard from "../components/AdminGuard.svelte";
</script>

<div class="space-y-4">
  <AdminGuard>
    <DevicesEx />
  </AdminGuard>
</div>
