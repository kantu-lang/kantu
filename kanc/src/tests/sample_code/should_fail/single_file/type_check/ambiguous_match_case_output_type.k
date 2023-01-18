type Nat {
    .o: Nat,
    .s(n: Nat): Nat,
}

type Eq(T: Type, a: T, b: T) {
    .refl(T: Type, c: T): Eq(T, c, c),
}

let foo = fun bar_(x: Nat): Nat {
    match
        match x {
            .s(problem) => Eq.refl(Nat, Nat.s(problem)),
            .o => Eq.refl(Nat, Nat.o),
        }
    {
        .refl(_U, _x) => Nat.o,
    }
};
