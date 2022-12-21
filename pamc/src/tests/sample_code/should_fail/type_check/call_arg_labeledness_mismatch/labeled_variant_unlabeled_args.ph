type Nat {
    .O: Nat,
    .S(pred~n: Nat): Nat,
}

let right = Nat.S(pred: Nat.O);
let wrong = Nat.S(Nat.O);
