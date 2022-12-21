type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

let O = Nat.O;

let right = Nat.S(O);
let wrong = Nat.S(pred: O);
