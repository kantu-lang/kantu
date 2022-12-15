type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

let f = fun (yellow~y: Nat, x: Nat): Nat {
    Nat.O
};
