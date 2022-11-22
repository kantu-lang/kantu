type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

let foo = match Nat.O {
    .O => Nat.O,
    .S => Nat.O,
};
