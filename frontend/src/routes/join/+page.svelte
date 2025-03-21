<div class="center">
  <form class="join" method="GET" action="/game">
    <h2>Skriv inn kode</h2>
    <input type="text" name="id">
    <input class="button" type="submit" value="Bli med!">
  </form>
</div>

<svelte:head>
  <title>Hvahoot - Bli med</title>
</svelte:head>

<style>
  .center {
    justify-content: center;
    height: 100%;
  }
  form {
    display: flex;
    flex-direction: column;
    gap: .5rem;
    box-shadow: var(--shadow);
    padding: .5rem;
    border-radius: .25rem;
    min-width: 14rem;
  }
  input {
    padding: .5rem;
    margin: 0;
    height: 3rem;
    box-sizing: border-box;
    border: .125rem solid var(--main);
  }
  h2 {
    text-align: center;
    border-bottom: .125rem solid var(--main);
    border-radius: .125rem;
  }
</style>