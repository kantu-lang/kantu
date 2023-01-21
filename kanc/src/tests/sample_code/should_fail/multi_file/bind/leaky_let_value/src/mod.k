pub type Nat {
    o: Nat,
    s(n: Nat): Nat,
}

let priv_S = Nat.s;

pub let(*) plus = fun plus(-a: Nat, b: Nat): Nat {
    match a {
        o => b,
        s(a') => priv_S(plus(a', b)),
    }
};
