type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

type Eq(T: Type, a: T, b: T) {
    .Refl(T: Type, c: T): Eq(T, c, c),
}

let foo = fun foo_(x: Nat): Nat {
    match x {
        .S(x') => fun inner(
            n: fun make_type(H: Eq(Nat, x, Nat.S(x'))): Type {
                Nat
            }(Eq.Refl(Nat, x'))
        ): Nat { n }(Nat.O),
        .O => Nat.O,
    }
};
