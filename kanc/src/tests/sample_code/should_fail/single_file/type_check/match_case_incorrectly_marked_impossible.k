type Nat {
    .o: Nat,
    .s(n: Nat): Nat,
}

let strict_pred = fun _(n: Nat): Nat {
    match n {
        .o => impossible,
        .s(n') => n',
    }
};
