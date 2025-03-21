<script lang="ts">
  import { isLoggedIn } from '$lib/token';
  import { onMount } from 'svelte';

  let loggedin: boolean | undefined = $state(undefined);

  onMount(() => {
    loggedin = isLoggedIn();
  });
</script>

<div class="headerpadding">
  <header>
    <a href="/">
      <img src="/logo.png" alt="Hvahoot" draggable="false">
    </a>
    <nav>
      {#if loggedin}
        <a href="/create" class="button">Create Hvahoot</a>
      {:else}
        <a class="button" href="/signup">Registrér</a>
        <a class="button" href="/login">Logg inn</a>
      {/if}
    </nav>
  </header>
</div>

<style>
  .headerpadding {
    height: 5rem;
    display: flex;
  }
  header {
    height: 5rem;
    padding: 1rem;
    box-sizing: border-box;
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    display: flex;
    flex-direction: row;
    gap: .5rem;
    justify-content: space-between;
    background-color: var(--bg);

    & > a {
      flex: 0;
    }
  }
  img {
    height: 100%;
    user-select: none;
  }
  nav {
    height: 100%;
    display: flex;
    flex-direction: row;
    gap: .5rem;
  }

  nav > a {
    height: 100%;
    box-sizing: border-box;
    display: flex;
    align-items: center;
    justify-content: center;
    border: .125rem solid var(--main);
    padding: .5rem;
    width: fit-content;
    text-decoration: none;
    color: inherit;
  }
</style>