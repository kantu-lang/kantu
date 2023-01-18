type Nat {
    .o: Nat,
    .s(n: Nat): Nat,
}

let o = Nat.o;

let right = Nat.s(o);
let wrong = Nat.s(pred: o);
