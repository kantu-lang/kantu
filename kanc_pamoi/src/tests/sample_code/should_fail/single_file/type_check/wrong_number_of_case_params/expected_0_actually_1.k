type Nat {
    o: Nat,
    s(n: Nat): Nat,
}

let foo = match Nat.o {
    o(n) => n,
    s(n) => n,
};
