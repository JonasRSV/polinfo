<script>
  import { Router, Link, Route } from "svelte-routing";
  import Home from "./routes/Home.svelte";
  import About from "./routes/About.svelte";

  export let url = "";
</script>

<style>
:global(body, html) {
  background-color: #121212;
  color: white;

}
</style>

<Router url="{url}">
  <nav>
    <Link to="/">Home</Link>
    <Link to="about">About</Link>
  </nav>
  <div>
    <Route path="about" component="{About}" />
    <Route path="/"><Home /></Route>
  </div>
</Router>
