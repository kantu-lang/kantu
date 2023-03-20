type Nat {
    o: Nat,
    s(n: Nat): Nat,
}

let f = fun _(~x: Nat, ~_: Nat): Nat {
    Nat.o
};
