pub type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

let priv_fun = fun _(n: Nat): Nat {
    Nat.S(n)
};

pub let value_uses_priv_item = priv_fun(Nat.O);
