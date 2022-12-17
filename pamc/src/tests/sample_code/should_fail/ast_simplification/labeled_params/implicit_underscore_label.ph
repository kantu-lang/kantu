type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

let f = fun _(~_: Nat): Nat {
    Nat.O
};
