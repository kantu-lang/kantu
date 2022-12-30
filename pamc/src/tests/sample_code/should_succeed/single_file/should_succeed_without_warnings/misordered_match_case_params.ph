//! TODO: We'll need to move this once we add
//! misorered match case param warnings

type Nat {
    .O: Nat,
    .S(_: Nat): Nat,
}

type Color {
    .C(~r: Nat, ~g: Nat, ~b: Nat): Color,
}

let identity = fun _(c: Color): Color {
    match c {
        .C(:b, :r, :g) => Color.C(:r, :g, :b),
    }
};

type ColorEq(a: Color, b: Color) {
    .Refl(c: Color): ColorEq(c, c),
}

let identity_correct = fun _(x: Color): ColorEq(x, identity(x)) {
    match x {
        .C(...) => ColorEq.Refl(x),
    }
};
