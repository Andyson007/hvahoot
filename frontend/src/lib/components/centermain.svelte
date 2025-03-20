<script lang="ts">
  const { children } = $props();
</script>

<div class="outermain">
  <main>
    {@render children()}
  </main>
</div>

<style>
  .outermain {
    display: flex;
    flex-direction: column;
    align-items: center;
  }
  main {
    max-width: 750px;
    width: 100%;
    box-sizing: border-box;
    padding: .5rem;
    display: flex;
    flex-direction: column;
    gap: .5rem;
  }
</style>