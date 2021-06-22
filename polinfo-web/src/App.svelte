<script>
  import { Router, Link, Route } from "svelte-routing";
  import Anforanden from "./routes/Anforanden.svelte";
  import About from "./routes/About.svelte";

  export let url = "";
</script>

<style>
:global(body, html) {
  background-color: #121212;
  color: white;

}

:global(a:visited) {
  color: skyblue;
}

:global(.word-clouds-anforanden) {
  color: rgb(255, 131, 21) !important;
}

:global(.about) {
  color: pink !important;

}

</style>

<Router url="{url}">
  <nav>
    <Link to="/" class="word-clouds-anforanden">Anforanden ☁ </Link>
    <Link to="about" class="about ml-5">About</Link>
  </nav>
  <div>
    <Route path="about" component="{About}" />
    <Route path="/"><Anforanden /></Route>
  </div>
</Router>
