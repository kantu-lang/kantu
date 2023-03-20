type Nat {
    o: Nat,
    s(n: Nat): Nat,
}

let f = fun _(~y: Nat, x: Nat): Nat {
    Nat.o
};
