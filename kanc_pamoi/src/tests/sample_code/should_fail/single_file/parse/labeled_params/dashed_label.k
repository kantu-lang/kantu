type Nat {
    o: Nat,
    s(n: Nat): Nat,
}

let f = fun _(-x~y: Nat): Nat {
    Nat.o
};
