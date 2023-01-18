type Nat {
    .o: Nat,
    .s(pred~n: Nat): Nat,
}

let pred = fun _(x: Nat): Nat {
    match x {
        .o => Nat.o,
        .s(:_) => x,
    }
};
