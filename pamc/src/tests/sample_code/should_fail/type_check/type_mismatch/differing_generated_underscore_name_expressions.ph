//! TODO: We'll need to move this once we add
//! misorered match case param warnings

type Nat {
    .O: Nat,
    .S(_: Nat): Nat,
}

type Color {
    .C(~r: Nat, ~g: Nat, ~b: Nat): Color,
}

let bad_identity = fun _(c: Color): Color {
    match c {
        .C(:b, :r, :g) => Color.C(r: g, :g, :b),
    }
};

type ColorEq(a: Color, b: Color) {
    .Refl(c: Color): ColorEq(c, c),
}

// Should fail
let identity_correct = fun _(x: Color): ColorEq(x, bad_identity(x)) {
    match x {
        .C(...) => ColorEq.Refl(x),
    }
};
