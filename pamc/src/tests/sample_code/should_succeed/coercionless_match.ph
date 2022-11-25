type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

type Eq(T: Type, a: T, b: T) {
    .Refl(T: Type, c: T): Eq(T, c, c),
}

let foo = fun bar_(x: Nat, y: Nat): Eq(Nat, y, y) {
    match
        match x {
            .S(_x') => Eq.Refl(Nat, y),
            .O => Eq.Refl(Nat, y),
        }
    {
        .Refl(_Nat, y2) => Eq.Refl(Nat, y2),
    }
};
