type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

let f = fun _(X~x: Nat, X~y: Nat): Nat {
    Nat.O
};
