<script lang="ts">
  import { page } from "$app/state";
  import Centermain from "$lib/components/centermain.svelte";
  import Errormessage from "$lib/components/errormessage.svelte";
  import Header from "$lib/components/header.svelte";
  import { onMount } from "svelte";

  interface question {
    question: string;
    answer?: number;
    answers: string[];
  }

  async function getNewHvahootID () {
    const resp = await fetch('/api/create/get', {
      method: 'GET'
    });

    const uuid = await resp.text();

    return uuid;
  }

  async function createHvahoot () {
    const resp = await fetch('/api/create', {
      method: 'POST',
      body: JSON.stringify({ name, uuid: id, questions }),
      headers: {
        'Content-Type': 'application/json'
      }
    });

    if (!resp.ok) {
      const { message } = await resp.json();
      error = message;
    }
  }

  let id: string | undefined = $state();
  let name: string = $state('');
  let questions: question[] = $state([]);
  let newHvahoot = $state(true);
  let error: string = $state('');

  function addQuestion () {
    questions.push({question: '', answer: undefined, answers: []})
    questions = questions;
  }

  function removeQuestion(index: number) {
    questions.splice(index, 1);
    questions = questions;
  }

  onMount(() => {
    // Get a new Quiz UUID
    const uuid = page.url.searchParams.get('id');
    if (uuid) {
      console.log('TODO: Load existing quiz');
      newHvahoot = false;
      id = uuid;
    }
    else {
      getNewHvahootID().then(uuid => { id = uuid; });
    }
  });
</script>

<Header/>
<Centermain>
  <Errormessage error={error}/>
  <section class="baseinfo">
    <input type="text" placeholder="Name" bind:value={name}>
  </section>
  <section class="questions">
    <h2>Questions</h2>
    <div class="questionlist">
      {#each questions as question, qindex}
        <div class="question">
          <div class="questionheader">
            <input type="text" placeholder="Question" bind:value={question.question}>
            <button class="button removequestion" onclick={() => removeQuestion(qindex)}>x</button>
          </div>
          <div class="answers">
            {#each question.answers as _, index}
              <div class="answer">
                <input type="text" placeholder="Answer #{index + 1}" bind:value={question.answers[index]}>
                <button class="correctradio {question.answer == index ? 'correct' : 'incorrect'}" aria-label={question.answer == index ? "Correct" : "Incorrect"} onclick={() => question.answer = index}></button>
              </div>
            {/each}
            <button onclick={() => question.answers.push('')} class="button">
              Add answer
            </button>
          </div>
        </div>
      {/each}
      <div class="center">
        <button onclick={addQuestion} class="button">Add question</button>
      </div>
    </div>
  </section>
  <button onclick={createHvahoot} class="button">{newHvahoot ? 'Create' : 'Update'} Hvahoot</button>
</Centermain>

<style>
  section {
    display: flex;
    flex-direction: column;
    gap: .5rem;
    padding: 1rem;
    border-radius: .25rem;
    box-shadow: var(--shadow);
  }
  input {
    border: .125rem solid var(--main);
    height: 3rem;
    box-sizing: border-box;
    padding: .5rem;
  }
  .questionlist {
    display: flex;
    flex-direction: column;
    gap: .5rem;
  }
  .question {
    border-radius: .25rem;
    padding: .5rem;
    border: .125rem solid var(--main);
    display: flex;
    flex-direction: column;
    gap: .5rem;
  }
  .answers {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: .5rem;

    .answer {
      display: flex;
      flex-direction: row;
      border: .125rem solid var(--main);
      
      & input {
        border: none;
        flex: 1;
      }
      & .correctradio {
        aspect-ratio: 1 / 1;
        box-sizing: border-box;
        margin: 0;
        padding: 0;
        border: none;
        border-radius: 100%;
        margin: .5rem;
        border: .125rem solid var(--main);

        background-color: #efefef;

        &.correct {
          background-color: var(--main);
        }
      }
    }
    &>button {
      height: 3.5rem;
    }
  }
  .questionheader {
    display: flex;
    flex-direction: row;
    gap: .5rem;

    & > input {
      flex: 1;
    }
    & > .removequestion {
      aspect-ratio: 1 / 1;
      border-color: var(--secondary);
      &:hover, &:active {
        background-color: var(--secondary);
      }
    }
  }
</style>