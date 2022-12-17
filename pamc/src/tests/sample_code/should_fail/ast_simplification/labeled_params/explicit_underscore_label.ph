type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

let f = fun _(_~x: Nat): Nat {
    Nat.O
};
