<script lang="ts">
  import { page } from "$app/state";
  import Errormessage from "$lib/components/errormessage.svelte";
  import Loading from "$lib/components/loading.svelte";
  import { onMount } from "svelte";

  type GameState = 'QUESTION' | 'QWAITING' | 'QRESULT' | 'USERNAME' | 'LOBBY';
  let currentstate: GameState = $state('USERNAME');

  const answerteases = ['Var du ikke litt VEL rask?', 'Tvi, tvi!', 'Bold move.', 'Du har all rett til å ta feil', 'Det svaret kan bli vanskelig å forsvare', 'La oss skje hva som skjer'];
  const wrongteases = ['Håper du får det til etter hvert', 'Dette kan bli kjedelig i lengden', 'Er Hvahooten vanskelig, eller er det et deg-problem?', 'Slapp av, du får det sikkert til neste gang'];
  let answerteasechoice: number = $state(0);

  let error: string = $state('');

  let points = 0;
  let newusername: string = $state('');
  let username: string = $state('');

  let currentquestion: { question: string, answers: string[] } | null = $state(null);
  let currentresult: { correct: boolean, points: number } | null = $state(null);

  let ws: WebSocket;

  onMount(() => {
    let quizid = page.url.searchParams.get('id');
    if (!quizid) window.location.href = '/join';
    
    ws = new WebSocket('/play/' + quizid);
    
    ws.addEventListener('close', () => {
      // window.location.href = '/join';
    });

    ws.addEventListener('message', ev => {
      const rawcontent = ev.data;
      let content: { type: string } & { [key: string]: any };
      
      try {
        content = JSON.parse(rawcontent);
      } catch {
        error = 'Could not parse content';
        return;
      }

      currentquestion = null;
      currentresult = null;
      
      switch (content.type) {
        case 'question':
          currentquestion = { question: content.question, answers: content.answers }
          currentstate = 'QUESTION';
          break;
        case 'qresult':
          currentresult = { points: content.points, correct: content.correct }
          points += currentresult.points;
          currentstate = 'QRESULT';
          break;
      }
    });
  });

  function submitAnswer (answer: number) {
    if(!ws) throw new Error('Unreachable code');

    ws.send(JSON.stringify({ type: 'answer', answer }));

    answerteasechoice = Math.floor(Math.random() * answerteases.length);
    currentstate = 'QWAITING';
  }

  function setUsername (ev: SubmitEvent) {
    ev.preventDefault();
    if (ws.readyState != ws.OPEN) return;

    ws.send(JSON.stringify({ type: 'username', username: newusername }));
    username = newusername;
    currentstate = 'LOBBY';
  }
</script>

<svelte:head>
  <title>Hvahoot - Quiz</title>
</svelte:head>

<div class="page">
  <header>
    <span class="points">{points}</span>
    {#if username}
      <span class="username">{username}</span>
    {/if}
  </header>
  <Errormessage error={error} />
  <main>
    {#if currentstate == 'USERNAME'}
      <div class="outerusername">
        <form class="usernameform" onsubmit={setUsername}>
          <input type="text" placeholder="Brukernavn" bind:value={newusername}>
          <input type="submit" value="Fortsett">
        </form> 
      </div>
    {:else if currentstate == 'LOBBY'}
      <div class="outerlobby">
        <span>Waiting for the game to begin</span>
      </div>
    {:else if currentstate == 'QUESTION'}
      {#if !currentquestion}
        <span>Loading...</span>
      {:else}
        <div class="outerq">
          <h2>{currentquestion.question}</h2>
          <div class="answers">
            {#each currentquestion.answers as answer, index}
              <button class="answer" onclick={() => submitAnswer(index)}>
                {answer}
              </button>
            {/each}
          </div>
        </div>
      {/if}
    {:else if currentstate == 'QWAITING'}
      <div class="outerqwaiting">
        <div class="qwaiting">
          <Loading />
          <span>{answerteases[answerteasechoice]}</span>
        </div>
      </div>
    {:else if currentstate == 'QRESULT'}
      <div class="outerqresult">
        <div class="qresult">
          {#if !currentresult}
            <Loading />
          {:else}
            <span class="points">{currentresult.points} points</span>
            {#if !currentresult.correct}
              <span>{wrongteases[Math.floor(Math.random() * wrongteases.length)]}</span>
            {/if}
          {/if}
        </div>
      </div>
    {/if}
  </main>
</div>

<style>
  header {
    padding: 1rem;
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
  }
  .username {
    background-color: #efefef;
    height: 3rem;
    padding: .5rem;
    box-sizing: border-box;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .points {
    height: 3rem;
    min-width: 4rem;
    padding: .5rem;
    box-sizing: border-box;
    background-color: #efefef;
    display: flex;
    justify-content: center;
    align-items: center;
  }
  .usernameform {
    display: flex;
    flex-direction: column;
    gap: .5rem;
    padding: .5rem;
    background-color: #efefef;
    width: 20rem;
    max-width: 100%;

    &>input {
      height: 3rem;
      padding: .5rem;
      margin: 0;
      box-sizing: border-box;
      font-size: .95rem;
      font: var(--font);
      border: .125rem solid var(--main);
      border-radius: 0;
      background-color: #ffffff;

      &[type="submit"] {
        &:hover, &:active {
          background-color: var(--main);
          color: #ffffff;
        }
      }
    }
  }
  .outerusername {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 1rem;
  }
  .outerq {
    flex: 1;
    display: flex;
    flex-direction: column;
    padding: .5rem;
    gap: .5rem;
  }
  .answers {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: .5rem;
    flex: 1;
    & > button {
      border: none;
      border-radius: .25rem;
      display: flex;
      align-items: center;
      justify-content: center;
    }
  }
  .outerlobby, .outerqwaiting  {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;

    &>span {
      font-style: italic;
    }
  }
  .qwaiting {
    display: flex;
    flex-direction: column;
    gap: .5rem;
    align-items: center;
    width: 500px;
    max-width: 100%;
    padding: 1rem;
  }
  .page {
    display: flex;
    flex-direction: column;
    height: 100%;
  }
  main {
    flex: 1;
    display: flex;
    flex-direction: column;
  }
</style>