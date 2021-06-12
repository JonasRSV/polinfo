<script>
import WordCloud from './WordCloud.svelte';
import PartiSymbol from "./PartiSymbol.svelte";
import * as d3 from "d3";
import { onMount } from 'svelte';
import DoubleRangeSlider from './DoubleRangeSlider.svelte'
import { getAnforandePrefixSum, avgPrefixSum, normCounts, normPrefixSum } from '../../api/anforande.js';

export let height;
export let affiliation;

var loading = true;

var wordCounts = []
var dates = []
var prefixSum = [];


let wordCountsFromPrefixSum = (prefixSum) => {
  let wc = [];
  for (var key in prefixSum) {
    wc.push({text: key, count: prefixSum[key]});
  }
  return wc;
}

let subtract = (lhs, rhs) => {
  let r = {};
  for (var key in lhs) {
    if (key in rhs) {
      r[key] = Math.max(lhs[key] - rhs[key], 0);
    } else {
      r[key] = lhs[key];
    }
  }

  return r;
}
 
 
let onIntervalPick = (fromIndex, toIndex) => {
  // in the integral eot is same as last date.
  
  // to subtract the 'beginning of time'
  fromIndex -= 1;
  toIndex -= 1;

  let end = prefixSum[toIndex];
  let avgEnd = avgPrefixSum[dates[toIndex + 1]];

  var start, avgStart;
  if (fromIndex < 0) {
    start = {};
    avgStart = {};
  } else {
    start = prefixSum[fromIndex];
    avgStart = avgPrefixSum[dates[fromIndex + 1]];
  }

  let normalizedCounts = subtract(end, start);
  let avgNormalizedCounts = subtract(avgEnd, avgStart);

  wordCounts = wordCountsFromPrefixSum(
  subtract(normalizedCounts, avgNormalizedCounts))
}


onMount(async () => {

  d3.select("#anforande-wordcloud-container")
  .style('height', height + "px")

  let res = await getAnforandePrefixSum(affiliation);
  prefixSum = normPrefixSum(res.counts);

  dates = ["beginning of time"];
  dates.push(...res.dates)


  onIntervalPick(0, dates.length - 1);


  loading = false;
})

</script>


<style>
  .loader,
.loader:before,
.loader:after {
  background: #ffffff;
  -webkit-animation: load1 1s infinite ease-in-out;
  animation: load1 1s infinite ease-in-out;
  width: 1em;
  height: 4em;
}
.loader {
  color: #ffffff;
  text-indent: -9999em;
  margin: 88px auto;
  position: relative;
  font-size: 11px;
  -webkit-transform: translateZ(0);
  -ms-transform: translateZ(0);
  transform: translateZ(0);
  -webkit-animation-delay: -0.16s;
  animation-delay: -0.16s;
}
.loader:before,
.loader:after {
  position: absolute;
  top: 0;
  content: '';
}
.loader:before {
  left: -1.5em;
  -webkit-animation-delay: -0.32s;
  animation-delay: -0.32s;
}
.loader:after {
  left: 1.5em;
}
@-webkit-keyframes load1 {
  0%,
  80%,
  100% {
    box-shadow: 0 0;
    height: 4em;
  }
  40% {
    box-shadow: 0 -2em;
    height: 5em;
  }
}
@keyframes load1 {
  0%,
  80%,
  100% {
    box-shadow: 0 0;
    height: 4em;
  }
  40% {
    box-shadow: 0 -2em;
    height: 5em;
  }
}

  .anforande-wordcloud-container {
    background-color: #202020;
    border-radius: 20px;
    padding: 5px;

  }
</style>


<div class="anforande-wordcloud-container" id="anforande-wordcloud-container">
  {#if loading}
    <div class="loader">Loading...</div>

  {:else}
    <WordCloud wordCounts={wordCounts} theme={affiliation} id={affiliation} height={height - 120}/> 
    <PartiSymbol parti={affiliation} />
    <DoubleRangeSlider id={affiliation} dates={dates} onIntervalPick={onIntervalPick}/>
  {/if}
</div>
