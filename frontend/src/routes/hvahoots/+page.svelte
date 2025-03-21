<script lang="ts">
  import Centermain from "$lib/components/centermain.svelte";
  import Header from "$lib/components/header.svelte";
  import { onMount } from "svelte";

  let hvahoots: { name: string, uuid: string }[] | undefined = $state();

  onMount(async () => {
    const resp = await fetch('/quizzes', {
      method: 'GET',
    });
    
    if (resp.ok) {
      hvahoots = await resp.json();
    }
  });
</script>

<Header />
<Centermain>
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
  <a href="/create" class="button">Create Hvahoot</a>
</Centermain>