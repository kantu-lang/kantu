type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

let strict_pred = fun _(n: Nat): Nat {
    match n {
        .O => impossible,
        .S(n') => n',
    }
};
