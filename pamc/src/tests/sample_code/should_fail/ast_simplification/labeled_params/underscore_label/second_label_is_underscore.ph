type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

let f = fun _(~x: Nat, ~_: Nat): Nat {
    Nat.O
};
