<script lang="ts">
  import { page } from "$app/state";
  import { onMount } from "svelte";

  let ws: WebSocket;
  let code: number | undefined = $state(undefined);

  let users: {uuid: string, name: string}[] = $state([]);
  let answered: number = 0;

  let currentstate: 'QUESTION' | 'SUMMARY' = $state('QUESTION');
  let question: { question: string, answers: {question: string, answers: string[]}[] } | undefined = $state(undefined);

  let scores: {[player_id: string]: {score: number, name: string}} | undefined = $state(undefined);

  onMount(() => {
    const uuid = page.url.searchParams.get('uuid');
    if (!uuid) window.location.href = '/';

    ws = new WebSocket('/play/host/'+uuid);

    ws.addEventListener('open', () => console.log('connected'));

    ws.addEventListener('message', ev => {
      const { data } = ev;      
      const json = JSON.parse(data);
      
      switch(json.type) {
        case 'code':
          code = json.code;
          break;
        case 'join':
          users.push({ uuid: json.id, name: json.username });
          break;
        case 'disconnect':
          const index = users.findIndex(u => u.uuid == json.id);
          if (index == -1) break;
          users.splice(index, 1);
          break;
        case 'answer':
          answered ++;
          break;
        case 'question':
          currentstate = 'QUESTION';
          question = { question: json.question, answers: json.answers };
          break;
        case 'summary':
          scores = json.scores;
          currentstate = 'SUMMARY';
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

<div class="page">
  <header>
    <span class="code">{code || 'Loading code'}</span>
    <button onclick={nextQ}>Next question</button>
  </header>
  <main>
    {#if currentstate == 'QUESTION'}
      {#if question}
        <h2 class="question">
          {question.question}
        </h2>
        <div class="answers">
          {#each question.answers as answer}
            <span>{answer}</span>
          {/each}
        </div>
      {/if}
    {:else if currentstate == 'SUMMARY'}
      <div class="outercenter">
        <div class="summary">
          {#if scores}
            {#each Object.keys(scores).sort((a, b) => scores ? scores[a].score - scores[b].score : 0) as uuid}
              <div class="player">
                <span class="name">
                  {scores[uuid].name}
                </span>
                <span class="score">
                  {scores[uuid].score}
                </span>
              </div>
            {/each}
          {/if}
        </div>
      </div>
    {/if}
  </main>
</div>

<style>
  header {
    height: 5rem;
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    padding: .5rem;
    box-sizing: border-box;
  }
  .code {
    font-size: 2rem;
  }
  main {
    flex: 1;
    gap: .5rem;
    padding: .5rem;
    display: flex;
    flex-direction: column;
  }
  .answers {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: .5rem;
    flex: 1;

    &>span {
      display: flex;
      align-items: center;
      justify-content: center;
      background-color: #efefef;
    }
  }
  .outercenter {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
  }
  .page {
    display: flex;
    flex-direction: column;
    height: 100%;
  }
</style>