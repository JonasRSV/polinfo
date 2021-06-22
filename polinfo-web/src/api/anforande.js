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

export function subtract(lhs, rhs) {
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
