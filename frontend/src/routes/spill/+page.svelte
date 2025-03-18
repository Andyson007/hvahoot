<script lang="ts">

  let code: string = $state('');

</script>

<div class="center">
  <form class="join">
    <h2>Skriv inn kode</h2>
    <input type="text" bind:value={code}>
    <input class="button" type="submit" value="Bli med!">
  </form>
</div>

<style>
  .center {
    justify-content: center;
    height: 100%;
  }
  form {
    display: flex;
    flex-direction: column;
    gap: .5rem;
    box-shadow: 0 0 2rem -.5rem var(--fg);
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