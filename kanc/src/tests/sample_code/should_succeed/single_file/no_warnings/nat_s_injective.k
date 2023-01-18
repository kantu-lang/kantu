type Nat {
    .o: Nat,
    .s(n: Nat): Nat,
}

type Eq(T: Type, x: T, y: T) {
    .refl(T: Type, z: T): Eq(T, z, z),
}

let nat_s_injective = fun _(
    a: Nat,
    b: Nat,
    H: Eq(Nat, Nat.s(a), Nat.s(b)),
): Eq(Nat, a, b) {
    match H {
        .refl(_Nat, _c) => Eq.refl(Nat, a),
    }
};
