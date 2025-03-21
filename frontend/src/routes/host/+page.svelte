<script lang="ts">
    import { page } from "$app/state";
  import { onMount } from "svelte";

  let ws: WebSocket;

  onMount(() => {
    const uuid = page.url.searchParams.get('uuid');
    if (!uuid) window.location.href = '/';

    ws = new WebSocket('/play/host/'+uuid);
  });

  function nextQ () {
    if (ws.readyState == ws.OPEN) {
      ws.send(JSON.stringify({ type: 'next' }));
    }
  }
</script>

<svelte:head>
  <title>Hvahoot - Arrangér spill</title>
</svelte:head>

<button onclick={nextQ}>Next question</button>