<script>
import { Container } from 'sveltestrap';
import { onMount } from 'svelte';
import MediaQuery from "./components/MediaQuery.svelte";
import AnforandeWordCloud from './components/AnforandeWordCloud.svelte';
import { getAnforandePrefixSum } from '../api/anforande.js';

/*let largeScreen = [["SD", "MP", "S", "M"], ["C", "V", "L", "KD"]];*/

let largeScreen = [["SD", "MP"], ["S", "M"], ["C", "V"], ["L", "KD"]];
let mediumScreen = [["SD", "MP"], ["S", "M"], ["C", "V"], ["L", "KD"]];
let smallScreen = [["SD"], ["MP"], ["S"], ["M"], ["C"], ["V"], ["L"], ["KD"]];

let render = false;

onMount(async function() {
  // Cache this.
  await getAnforandePrefixSum("ALL");


  render = true;


})

</script>


<style>
</style>


<Container fluid>
  <header class="mb-5 text-center">
    <h1 class="text-center">Anföranden</h1>
    <p> Wordsclouds of <a href="https://data.riksdagen.se/data/anforanden/"> https://data.riksdagen.se/data/anforanden/ </a>
  </header>

  {#if render}
  <MediaQuery query="(min-width: 1281px)" let:matches>
    {#if matches}
      {#each largeScreen as block}
        <div class="d-flex flex-row mb-5">
          {#each block as parti}
            <div class="d-flex col-6 flex-column px-md-3"> 
              <AnforandeWordCloud height={450} affiliation={parti} />
            </div>
          {/each}
        </div>
      {/each}
    {/if}
  </MediaQuery>

  <MediaQuery query="(min-width: 900px) and (max-width: 1280px)" let:matches>
    {#if matches}
      {#each mediumScreen as block}
        <div class="d-flex flex-row mb-5">
          {#each block as parti}
            <div class="d-flex col-6 flex-column px-md-3"> 
              <AnforandeWordCloud height={450} affiliation={parti} />
            </div>
          {/each}
        </div>
      {/each}
    {/if}
  </MediaQuery>

  <MediaQuery query="(max-width: 900px)" let:matches>
    {#if matches}
      {#each smallScreen as block}
        <div class="d-flex flex-row mb-5">
          {#each block as parti}
            <div class="d-flex col-12 flex-column"> 
              <AnforandeWordCloud height={500} affiliation={parti} />
            </div>
          {/each}
        </div>
      {/each}
    {/if}
  </MediaQuery>
  {:else}
    <div>Caching a few things...</div>
  {/if}
</Container>
