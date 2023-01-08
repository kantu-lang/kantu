type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

type Rgb {
    .C(~r: Nat, ~g: Nat, ~b: Nat): Rgb,
}

let O = Nat.O;
let rgb = Rgb.C;

let right = rgb(r: O, g: O, b: O);
let wrong = rgb(r: O);
