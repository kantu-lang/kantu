type Nat {
    .o: Nat,
    .s(n: Nat): Nat,
}

type Eq(T: Type, a: T, b: T) {
    .refl(T: Type, t: T): Eq(T, t, t),
}

let foo = match Eq.refl(Nat, Nat.o) {
    .refl(o) => o,
};
