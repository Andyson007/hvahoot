<script lang="ts">
  let { error }: { error: string } = $props();
</script>

{#if error}
  <span class="error">{error}</span>
{/if}

<style>
</style>