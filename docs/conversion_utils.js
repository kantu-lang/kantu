// Utils for converting JS representations of
// Kantu types (e.g., Nat, List)
// to their native analogues (e.g., number, Array).

function toNum(nat) {
  if (nat.length === 1) {
    return 0;
  }
  if (nat.length === 2) {
    return 1 + toNum(nat[1]);
  }
  throw { badNat: nat };
}

function toArr(list) {
  if (list.length === 2) {
    return [];
  }
  if (list.length === 4) {
    return [list[2]].concat(toArr(list[3]));
  }
  throw { badList: list };
}
