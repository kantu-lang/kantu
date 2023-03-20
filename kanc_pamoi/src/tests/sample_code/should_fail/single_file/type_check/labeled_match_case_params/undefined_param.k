type Nat {
    o: Nat,
    s(_: Nat): Nat,
}

type Color {
    c(~r: Nat, ~g: Nat, ~b: Nat): Color,
}

let _1 = Nat.s(Nat.o);
let white = Color.c(r: _1, g: _1, b: _1);

let _0 = match white {
    c(:alpha, r: y, hue: x, ...) => Nat.o,
};
