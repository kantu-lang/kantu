pub type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

let(*) _0 = Nat.O;
