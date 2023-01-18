type Nat {
    .o: Nat,
    .s(n: Nat): Nat,
}

let f = fun _(~x: Nat, x~y: Nat): Nat {
    Nat.o
};
