type Nat {
    .o: Nat,
    .s(n: Nat): Nat,
}

let f = fun _(_~x: Nat): Nat {
    Nat.o
};
