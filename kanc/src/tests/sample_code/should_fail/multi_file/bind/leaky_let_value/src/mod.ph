pub type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

let priv_S = Nat.S;

pub let(*) plus = fun plus(-a: Nat, b: Nat): Nat {
    match a {
        .O => b,
        .S(a') => priv_S(plus(a', b)),
    }
};
