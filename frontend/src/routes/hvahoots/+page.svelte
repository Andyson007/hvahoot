<script lang="ts">
  import { onMount } from "svelte";

  let hvahoots: { name: string, uuid: string }[] | undefined = $state();

  onMount(async () => {
    const resp = await fetch('/api/quizzes', {
      method: 'GET',
    });
    
    if (resp.ok) {
      hvahoots = await resp.json();
    }
  });
</script>

<div class="hvahoots">
  {#if hvahoots}
    {#each hvahoots as hvahoot}
      <a class="hvahoot button" href="/host?uuid={hvahoot.uuid}">
        {hvahoot.name}
      </a>
    {/each}
  {:else}
    <span>Loading</span>
  {/if}
</div>