pub type Nat {
    o: Nat,
    s(n: Nat): Nat,
}

let(*) _0 = Nat.o;
