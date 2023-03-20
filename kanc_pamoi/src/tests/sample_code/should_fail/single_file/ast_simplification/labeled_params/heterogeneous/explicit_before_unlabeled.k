type Nat {
    o: Nat,
    s(n: Nat): Nat,
}

let f = fun _(yellow~y: Nat, x: Nat): Nat {
    Nat.o
};
