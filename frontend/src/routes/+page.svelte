<script>
  import Centermain from "$lib/components/centermain.svelte";
  import Header from "$lib/components/header.svelte";

</script>

<svelte:head>
  <title>Hvahoot - Quiz som du aldri har sett det</title>
</svelte:head>

<Header />
<Centermain>
  <h1>Hvahoot - Quiz som du aldri har sett det</h1>
  <p>
    Hvahoot er et quiz-spill der du kjemper mot andre i en stor kamp om kunnskap. Det er om å gjøre å være den mest kunnskapsrike - og kun én står igjen på toppen til slutt.
  </p>
  <div class="center">
    <a href="/join" class="button">Spill nå!</a>
  </div>
</Centermain>

<style>
</style>