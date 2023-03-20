type Nat {
    o: Nat,
    s(n: Nat): Nat,
}

let f = fun _(~z: Nat, ~z: Nat): Nat {
    Nat.o
};
