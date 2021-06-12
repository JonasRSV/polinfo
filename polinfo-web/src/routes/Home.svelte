<script>
  import { onMount } from 'svelte';
  import Anforanden from './components/Anforanden.svelte';
  import { cacheAvgPrefixSum } from './../api/anforande.js';

let render = false;


onMount(async () => {
  await cacheAvgPrefixSum();
  render = true;
});


</script>


<style>
</style>

<main class="d-flex flew-row justify-content-center">
  {#if render}
    <Anforanden />
  {:else}
    <div>Calculating stuff..</div>
  {/if}
</main>
