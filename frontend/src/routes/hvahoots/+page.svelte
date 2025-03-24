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
      <div class="hvahoot">
        <a class="button outername" href="/host?uuid={hvahoot.uuid}">
          <span class="name">
            {hvahoot.name}
          </span>
        </a>
        <a href="/create?id={hvahoot.uuid}" class="button">Edit</a>
      </div>
      {/each}
    {:else}
      <span>Loading</span>
    {/if}
  </div>
  <a href="/create" class="button">Create Hvahoot</a>
</Centermain>

<style>
  .outername {
    overflow: hidden;
  }
  .name {
    text-overflow: ellipsis;
    overflow: hidden;
    white-space: nowrap;
  }
  .hvahoot {
    display: flex;
    flex-direction: row;
    gap: .5rem;
    flex-grow: 0;
  }
  .hvahoots {
    display: flex;
    flex-direction: column;
    gap: .5rem;
    flex-grow: 0;
  }
</style>