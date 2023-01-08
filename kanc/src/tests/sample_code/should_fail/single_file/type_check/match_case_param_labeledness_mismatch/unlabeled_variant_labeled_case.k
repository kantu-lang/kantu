type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

let plus = fun plus(-a: Nat, b: Nat): Nat {
    match a {
        .O => b,
        .S(:pred) => Nat.S(plus(pred, b)),
    }
};
