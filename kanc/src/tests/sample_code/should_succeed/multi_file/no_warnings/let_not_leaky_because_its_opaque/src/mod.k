pub type Nat {
    .o: Nat,
    .s(n: Nat): Nat,
}

let priv_fun = fun _(n: Nat): Nat {
    Nat.s(n)
};

pub let value_uses_priv_item = priv_fun(Nat.o);
