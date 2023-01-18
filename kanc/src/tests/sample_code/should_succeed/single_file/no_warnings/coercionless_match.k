type Nat {
    .o: Nat,
    .s(n: Nat): Nat,
}

type Eq(T: Type, a: T, b: T) {
    .refl(T: Type, c: T): Eq(T, c, c),
}

let foo = fun bar_(x: Nat, y: Nat): Eq(Nat, y, y) {
    match
        match x {
            .s(_x') => Eq.refl(Nat, y),
            .o => Eq.refl(Nat, y),
        }
    {
        .refl(_Nat, y2) => Eq.refl(Nat, y2),
    }
};
