type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

let f = fun _(y~x: Nat, ~y: Nat): Nat {
    Nat.O
};
