<script>
import { afterUpdate } from 'svelte';
import cloud from "d3-cloud";
import * as d3 from "d3";
import { onMount } from 'svelte';

export let id;
export let wordCounts;
export let theme;
export let height;

let width;

export let largestWord = 30;

let wordcloud; 

let colors = {
  "SD": ["rgba(248, 255, 36, 1.0)", "rgba(244, 252, 0, 1.0)", "rgba(251, 255, 128, 1.0)"],
  "S": ["rgba(255, 77, 103, 1.0)", "rgba(250, 112, 134, 1.0)", "rgba(255, 140, 147, 1.0)"],
  "MP": ["rgba(156, 255, 140, 1.0)", "rgba(112, 242, 92, 1.0)", "rgba(65, 255, 36, 1.0)"],
  "M": ["rgba(120, 172, 255, 1.0)", "rgba(135, 169, 224, 1.0)", "rgba(153, 192, 255, 1.0)"],
  "C": ["rgba(156, 255, 140, 1.0)", "rgba(112, 242, 92, 1.0)", "rgba(65, 255, 36, 1.0)"],
  "V": ["rgba(255, 77, 103, 1.0)", "rgba(250, 112, 134, 1.0)", "rgba(255, 140, 147, 1.0)"],
  "KD": ["rgba(120, 172, 255, 1.0)", "rgba(135, 169, 224, 1.0)", "rgba(153, 192, 255, 1.0)"],
  "L": ["rgba(120, 172, 255, 1.0)", "rgba(135, 169, 224, 1.0)", "rgba(153, 192, 255, 1.0)"]
}[theme]



var componentIsMounted = false;
var words = [];
let getWords = () => {
  var max = 0;

  wordCounts.forEach(obj => {
    if (obj.count > max) {
      max = obj.count;
    }
  })

  /*console.log("word counts", wordCounts);*/

  let norm = largestWord / max;
  return wordCounts.map(obj => {
    return {text: obj.text, size: Math.round(obj.count * norm)};
  }).filter((w) => w.size > 0);

}

let render = (words) => {
  /*console.log("rendering", words);*/
  var layout = cloud()
    .size([width, height])
    .words(words)
    .padding(5)
    .rotate(function() { return (Math.random() - 0.5) * 70; })
    .font("Impact")
    .fontSize(function(d) { 
      /*console.log("setting fontsize on", d);*/
      return d.size; 
    })
    
    .on("end", draw);

  layout.start();

  function draw(words) {
    /*console.log("drawing", words);*/
    d3.select("#" + id).append("svg")
        .attr("width", layout.size()[0])
        .attr("height", layout.size()[1])
      .append("g")
        .attr("transform", "translate(" + layout.size()[0] / 2 + "," + layout.size()[1] / 2 + ")")
      .selectAll("text")
        .data(words)
      .enter().append("text")
        .style("font-size", function(d) { return d.size + "px"; })
      .style("fill", function (_) { 
        let index = Math.round(Math.random() * (colors.length - 1));

        return colors[index];
      })
        .style("font-family", "Impact")
        .attr("text-anchor", "middle")
        .attr("transform", function(d) {
          return "translate(" + [d.x, d.y] + ")rotate(" + d.rotate + ")";
        })
        .text(function(d) { return d.text; });
    }
}

afterUpdate(() => {
  words = getWords();
  /*console.log("Words", words);*/

  if (componentIsMounted) {
    d3.select("#" + id).select("svg").remove();
    render(words);
  }
})

onMount(async () => {
  let rect =  wordcloud.getBoundingClientRect();
  width = rect.width;

  words = getWords();
  /*console.log("Words", words);*/


  render(words);


  componentIsMounted = true;
});

</script>


<style>

</style>

<div bind:this={wordcloud} id={id} width="{width}px" class="word-cloud d-flex flex-row justify-content-center"></div>
