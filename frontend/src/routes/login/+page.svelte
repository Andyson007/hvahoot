<script lang="ts">
    import Errormessage from "$lib/components/errormessage.svelte";

  let username: string = $state('');
  let password: string = $state('');

  let error: string = $state('');

  async function login () {
    const resp = await fetch('/api/login', {
      method: 'POST',
      body: JSON.stringify({
        username, password
      })
    });

    if (!resp.ok) error = (await resp.json().catch(() => {return {message: 'An error occured while parsing the response'}})).message;
  }
</script>

<main>
  <form onsubmit={() => login()}>
    <Errormessage error={error} />
    <input type="text" bind:value={username}>
    <input type="password" bind:value={password}>
    <input class="button" type="submit" value="Logg inn">
  </form>
</main>

<style>

form {
  display: flex;
  flex-direction: column;
  gap: .5rem;
  width: 14rem;
  padding: .5rem;
  box-shadow: 0 0 2rem -.5rem var(--fg);
  border-radius: .25rem;
}

input {
  height: 3rem;
  border: .125rem solid var(--main);
  padding: .5rem;
  margin: 0;
  font-size: 1rem;
  box-sizing: border-box;
}

main {
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}

</style>