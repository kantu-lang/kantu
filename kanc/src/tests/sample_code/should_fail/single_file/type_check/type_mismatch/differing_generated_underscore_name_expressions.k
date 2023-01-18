//! TODO: We'll need to move this once we add
//! misordered match case param warnings

type Nat {
    .o: Nat,
    .s(_: Nat): Nat,
}

type Color {
    .c(~r: Nat, ~g: Nat, ~b: Nat): Color,
}

let bad_identity = fun _(c: Color): Color {
    match c {
        .c(:b, :r, :g) => Color.c(r: g, :g, :b),
    }
};

type ColorEq(a: Color, b: Color) {
    .refl(c: Color): ColorEq(c, c),
}

// Should fail
let identity_correct = fun _(x: Color): ColorEq(x, bad_identity(x)) {
    match x {
        .c(...) => ColorEq.refl(x),
    }
};
