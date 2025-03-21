<script lang="ts">
  import { page } from "$app/state";
  import { onMount } from "svelte";

  let ws: WebSocket;
  let code: number | undefined = $state(undefined);

  onMount(() => {
    const uuid = page.url.searchParams.get('uuid');
    if (!uuid) window.location.href = '/';

    ws = new WebSocket('/play/host/'+uuid);

    ws.addEventListener('message', ev => {
      const { data } = ev.data;
      const json = JSON.parse(data);
      
      switch(json.type) {
        case 'code':
          code = json.code;
          break;
      }
    })
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
<span class="code">{code || 'Loading code'}</span>

<style>
  .code {
    font-size: 2rem;
  }
</style>