type Nat {
    .o: Nat,
    .s(n: Nat): Nat,
}

let f = fun _(y~x: Nat, ~y: Nat): Nat {
    Nat.o
};
