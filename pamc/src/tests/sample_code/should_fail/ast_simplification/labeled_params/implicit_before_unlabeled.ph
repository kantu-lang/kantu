type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

let f = fun (~y: Nat, x: Nat): Nat {
    Nat.O
};
