type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

let foo = match Nat.O {
    .O(n) => n,
    .S(n) => n,
};
