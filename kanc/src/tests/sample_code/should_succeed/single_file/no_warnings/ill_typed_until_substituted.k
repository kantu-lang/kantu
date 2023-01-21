type Nat {
    o: Nat,
    s(n: Nat): Nat,
}

type Eq(T: Type, a: T, b: T) {
    refl(T: Type, c: T): Eq(T, c, c),
}

let foo = fun foo_(x: Nat): Nat {
    match x {
        s(x') => fun inner(
            m_thisParamTypeShouldBeChecked: Nat,
            n_thisParamTypeShouldBeChecked:
                fun make_type(H: Eq(Nat, x, Nat.s(x'))): Type {
                    Nat
                }(Eq.refl(Nat, x))
        ): Nat { n_thisParamTypeShouldBeChecked }(Nat.o, Nat.o),
        o => Nat.o,
    }
};
