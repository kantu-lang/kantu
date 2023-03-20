type U1 {
    c: U1,
}

type U2 {
    c: U2,
}

type TypeEq(_: Type, _: Type) {
    refl(T: Type): TypeEq(T, T),
}

let foo = fun _(H: TypeEq(U1, U2)): U1 {
    match H {
        refl(_) => U1.c,
    }
};
