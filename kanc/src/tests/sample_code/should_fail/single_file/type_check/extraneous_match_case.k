type Bool {
    .true: Bool,
    .false: Bool,
}

type Nat {
    .o: Nat,
    .s(n: Nat): Nat,
}

let foo = match Bool.true {
    .true => Nat.o,
    .false => Nat.s(Nat.o),
    .maybe => Nat.s(Nat.s(Nat.o)),
};
