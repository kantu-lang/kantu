type Nat {
    .o: Nat,
    .s(~pred: Nat): Nat,
}

let plus = fun plus(-a: Nat, b: Nat): Nat {
    match a {
        .o => b,
        .s(a') => Nat.s(pred: plus(a', b)),
    }
};
