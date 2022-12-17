type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

let f = fun _(~z: Nat, ~z: Nat): Nat {
    Nat.O
};
