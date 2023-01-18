type Nat {
    .o: Nat,
    .s(n: Nat): Nat,
}

let foo = fun _(-n: Nat): Nat {
    match n {
        .o => Nat.o,
        .s(n') => _(n'),
    }
};
