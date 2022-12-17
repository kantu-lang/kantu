type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

let f = fun _(x: Nat, ~y: Nat): Nat {
    Nat.O
};
