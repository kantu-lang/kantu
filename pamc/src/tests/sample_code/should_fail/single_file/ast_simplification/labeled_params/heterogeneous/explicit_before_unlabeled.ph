type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

let f = fun _(yellow~y: Nat, x: Nat): Nat {
    Nat.O
};
