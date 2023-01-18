type Nat {
    .o: Nat,
    .s(pred~n: Nat): Nat,
}

let right = Nat.s(pred: Nat.o);
let wrong = Nat.s(Nat.o);
