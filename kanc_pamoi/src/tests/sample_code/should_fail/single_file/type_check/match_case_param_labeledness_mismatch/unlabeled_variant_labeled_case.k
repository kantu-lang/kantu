type Nat {
    o: Nat,
    s(n: Nat): Nat,
}

let plus = fun plus(-a: Nat, b: Nat): Nat {
    match a {
        o => b,
        s(:pred) => Nat.s(plus(pred, b)),
    }
};
