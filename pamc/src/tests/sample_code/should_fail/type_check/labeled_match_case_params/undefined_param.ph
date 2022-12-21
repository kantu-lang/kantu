type Nat {
    .O: Nat,
    .S(_: Nat): Nat,
}

type Color {
    .C(~r: Nat, ~g: Nat, ~b: Nat): Color,
}

let _1 = Nat.S(Nat.O);
let white = Color.C(r: _1, g: _1, b: _1);

let _0 = match white {
    .C(:alpha, r: y, hue: x, ...) => Nat.O,
};
