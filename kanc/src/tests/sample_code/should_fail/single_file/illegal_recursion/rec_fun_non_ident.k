type Nat {
    o: Nat,
    s(n: Nat): Nat,
}

let x = fun x(-a: Nat, b: Nat): Nat {
    x(Nat.o, b)
};
