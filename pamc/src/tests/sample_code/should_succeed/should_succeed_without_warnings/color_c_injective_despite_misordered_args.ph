//! TODO: We'll need to move this to the "succeed _with_ warnings"
//! directory, after we implement misordered arg warnings.

type Nat {
    .O: Nat,
    .S(_: Nat): Nat,
}

type Color {
    .C(~r: Nat, ~g: Nat, ~b: Nat): Color,
}

type Eq(T: Type, a: T, b: T) {
    .Refl(T: Type, t: T): Eq(T, t, t),
}

type Triple(T: Type, U: Type, V: Type) {
    .C(T: Type, U: Type, V: Type, t: T, u: U, v: V): Triple(T, U, V),
}

let color_c_injective = fun _(
    r: Nat,
    g: Nat,
    b: Nat,
    r2: Nat,
    g2: Nat,
    b2: Nat,
    H: Eq(Color, Color.C(:r, :g, :b), Color.C(b: b2, g: g2, r: r2)),
): Triple(Eq(Nat, r, r2), Eq(Nat, g, g2), Eq(Nat, b, b2)) {
    match H {
        .Refl(_Color, _color) =>
            Triple.C(
                Eq(Nat, r, r2),
                Eq(Nat, g, g2),
                Eq(Nat, b, b2),
                
                Eq.Refl(Nat, r),
                Eq.Refl(Nat, g),
                Eq.Refl(Nat, b),
            ),
    }
};
