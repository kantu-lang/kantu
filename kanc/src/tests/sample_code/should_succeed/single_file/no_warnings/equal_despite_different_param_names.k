type Nat {
    .o: Nat,
    .s(n: Nat): Nat,
}

let ident1 = fun _(x: Nat): Nat { x };
let ident2 = fun _(y: Nat): Nat { y };

type Eq(T: Type, a: T, b: T) {
    .refl(T: Type, c: T): Eq(T, c, c),
}

type Unit { .c: Unit }

let ident1_equals_ident2 = fun _(_: Unit): Eq(
    forall(z: Nat) { Nat },
    ident1,
    ident2,
) {
    Eq.refl(forall(z: Nat) { Nat }, ident1)
};
