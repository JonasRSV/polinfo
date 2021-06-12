import { host } from "./api";


let cache = { }
export async function getAnforandePrefixSum(affiliation) {
  if (affiliation in cache) {
    return cache[affiliation];
  }

  let response = await fetch(host + "/anforanden", {
    method: 'POST',
    headers: {
          'Accept': 'application/json',
          'Content-Type': 'application/json'
        },
    body: JSON.stringify({
      'affiliation': affiliation
    })
  })

  cache[affiliation] = await response.json()
    
  return cache[affiliation];
}

let union = (lhs, rhs) => {
  //console.log("lhs", lhs, "rhs", rhs);
  let keys = Object.keys(lhs).concat(Object.keys(rhs))
  let uniqueKeys = keys.filter((v, i, s) => s.indexOf(v) == i);

  let u = {};
  uniqueKeys.forEach(k => {
    u[k] = 0;

    if (k in lhs) {
      u[k] += lhs[k]
    }

    if (k in rhs) {
      u[k] += rhs[k]
    }

  });

  return u;
}

/**
 * Normalize a prefix sum 
 *
 * @param {list of dicts} prefixSum - the sequence of counts 
 * @return {list of dicts} same as input but normalized
 */
export let normPrefixSum = (prefixSum) => {
  let normalized = [];

  var sum = 0;
  for (var i = 0; i < prefixSum.length; i++) {
    for (var key in prefixSum[i]) {
      sum += prefixSum[i][key];
    }
  }

  for (var i = 0; i < prefixSum.length; i++) {
    let norm = {}
    for (var key in prefixSum[i]) {
      norm[key] = prefixSum[i][key] / sum;
    }

    normalized.push(norm);
  }

  return normalized
}

/**
 * Normalize a single count
 *
 * @param {a dict} counts - a mapping from words to counts
 * @return {a dict} same as input
 */
export let normCounts = (counts) => {
  let normalized = {};

  var sum = 0;
  for (var key in counts) {
    sum += counts[key];
  }

  for (var key in counts) {
    normalized[key] = counts[key] / sum;
  }

  return normalized
}

export let avgPrefixSum = undefined;
export async function cacheAvgPrefixSum() {
  let parties = ["SD", "MP", "S", "M", "C", "V", "L", "KD"];
  let averages = {};

  for (var j = 0; j < parties.length; j++) {
    let party = parties[j];
    let prefixSum = await getAnforandePrefixSum(party) 

    prefixSum.counts = normPrefixSum(prefixSum.counts);

    for (var i = 0; i < prefixSum.dates.length; i++) {
      if (!(prefixSum.dates[i] in averages)) {
        averages[prefixSum.dates[i]] = {};
      }

      averages[prefixSum.dates[i]] = union(averages[prefixSum.dates[i]], prefixSum.counts[i]);
    }
  }


  for (var key in averages) {
    for (var word in averages[key]) {
      averages[key][word] /= parties.length;
    }
  }

  avgPrefixSum = averages;
}

