type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

let foo = fun _(-n: Nat): Nat {
    match n {
        .O => Nat.O,
        .S(n') => _(n'),
    }
};
