// Utils for converting JS representations of
// Kantu types (e.g., Nat, List)
// to their native analogues (e.g., number, Array).

function pnatToNum(nat) {
  if (nat.length === 1) {
    return 0;
  }
  if (nat.length === 2) {
    return 1 + pnatToNum(nat[1]);
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

function intToNum(int) {
  if (int[0] === "neg") {
    return -posToNum(int[1]);
  }
  if (int[0] === "nat") {
    return natToNum(int[1]);
  }
  throw { badInt: int };
}

function natToNum(nat) {
  if (nat[0] === "zero") {
    return 0;
  }
  if (nat[0] === "pos") {
    return posToNum(nat[1]);
  }
  throw { badNat: nat };
}

function posToNum(pos) {
  return parseInt(posToBitString(pos), 2);
}

function posToBitString(pos) {
  if (pos[0] === "one") {
    return "1";
  }
  if (pos[0] === "extend") {
    const left = posToBitString(pos[1]);
    const right = String(bitToNum(pos[2]));
    return left + right;
  }
  throw { badPos: pos };
}

function bitToNum(bit) {
  if (bit[0] === "zero") {
    return 0;
  }
  if (bit[0] === "one") {
    return 1;
  }
  throw { badBit: bit };
}
