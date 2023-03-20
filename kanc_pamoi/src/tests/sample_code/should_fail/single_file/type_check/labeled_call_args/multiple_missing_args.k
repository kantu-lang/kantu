type Nat {
    o: Nat,
    s(n: Nat): Nat,
}

type Rgb {
    c(~r: Nat, ~g: Nat, ~b: Nat): Rgb,
}

let O = Nat.o;
let rgb = Rgb.c;

let right = rgb(r: O, g: O, b: O);
let wrong = rgb(r: O);
