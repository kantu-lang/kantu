type U1 {
    .C: U1,
}

type U2 {
    .C: U2,
}

type TypeEq(_: Type, _: Type) {
    .Refl(T: Type): TypeEq(T, T),
}

let foo = fun _(H: TypeEq(U1, U2)): U1 {
    match H {
        .Refl(_) => U1.C,
    }
};
