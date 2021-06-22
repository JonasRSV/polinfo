export function div_by_scalar(counts, scalar) {
  let res = {}
  for (var key in counts) {
    res[key] = counts[key] / scalar;
  }

  return res;
}

export function div_by_counts(rhs, lhs) {
  let res = {}
  for (var key in rhs) {
    res[key] = rhs[key] / lhs[key];
  }

  return res;
}

export function sub_by_counts(rhs, lhs) {
  let res = {}
  for (var key in rhs) {
    res[key] = rhs[key] - lhs[key];
  }

  return res;
}
