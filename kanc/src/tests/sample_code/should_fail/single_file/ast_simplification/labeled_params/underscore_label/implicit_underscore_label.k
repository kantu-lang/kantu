type Nat {
    .o: Nat,
    .s(n: Nat): Nat,
}

let f = fun _(~_: Nat): Nat {
    Nat.o
};
