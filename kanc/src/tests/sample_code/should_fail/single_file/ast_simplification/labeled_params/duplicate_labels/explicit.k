type Nat {
    .o: Nat,
    .s(n: Nat): Nat,
}

let f = fun _(X~x: Nat, X~y: Nat): Nat {
    Nat.o
};
