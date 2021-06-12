<script>
  export let parti;

</script>


<style>

.logo {
  background-position: center !important;
  background-size: contain !important;
  background-repeat: no-repeat !important;
  background-origin: content-box !important;
  padding: 15px !important;
  height: 100px !important;
}

.socialdemokraterna { background: url("/assets/icons/socialdemokraterna.svg"); }
.moderaterna { background: url("/assets/icons/moderaterna.svg"); }
.sverigedemokraterna { background: url("/assets/icons/sverigedemokraterna.svg"); }
.miljopartiet { background: url("/assets/icons/miljopartiet.svg"); }
.liberalerna { background: url("/assets/icons/liberalerna.svg"); }
.centerpartiet { background: url("/assets/icons/centerpartiet.svg"); }
.vansterpartiet { background: url("/assets/icons/vansterpartiet.svg"); }
.kristdemokraterna { background: url("/assets/icons/kristdemokraterna.svg"); }

</style>

{#if parti == "S"}
  <div class="socialdemokraterna logo"></div>
{:else if parti == "M"}
  <div class="moderaterna logo"></div>
{:else if parti == "SD"}
  <div class="sverigedemokraterna logo"></div>
{:else if parti == "L"}
  <div class="liberalerna logo"></div>
{:else if parti == "MP"}
  <div class="miljopartiet logo"></div>
{:else if parti == "C"}
  <div class="centerpartiet logo"></div>
{:else if parti == "V"}
  <div class="vansterpartiet logo"></div>
{:else if parti == "KD"}
  <div class="kristdemokraterna logo"></div>
{:else}
  <div class="logo unknown">unknown parti {parti}</div>
{/if}
