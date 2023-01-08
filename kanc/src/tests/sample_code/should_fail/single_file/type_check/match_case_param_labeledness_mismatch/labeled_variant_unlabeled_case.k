type Nat {
    .O: Nat,
    .S(~pred: Nat): Nat,
}

let plus = fun plus(-a: Nat, b: Nat): Nat {
    match a {
        .O => b,
        .S(a') => Nat.S(pred: plus(a', b)),
    }
};
