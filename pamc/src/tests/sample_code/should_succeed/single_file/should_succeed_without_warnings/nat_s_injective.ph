type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

type Eq(T: Type, x: T, y: T) {
    .Refl(T: Type, z: T): Eq(T, z, z),
}

let nat_s_injective = fun _(
    a: Nat,
    b: Nat,
    H: Eq(Nat, Nat.S(a), Nat.S(b)),
): Eq(Nat, a, b) {
    match H {
        .Refl(_Nat, _c) => Eq.Refl(Nat, a),
    }
};
