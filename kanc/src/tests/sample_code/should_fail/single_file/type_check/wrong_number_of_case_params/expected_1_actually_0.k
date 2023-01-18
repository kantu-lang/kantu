type Nat {
    .o: Nat,
    .s(n: Nat): Nat,
}

let foo = match Nat.o {
    .o => Nat.o,
    .s => Nat.o,
};
