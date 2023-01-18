type Nat {
    .o: Nat,
    .s(n: Nat): Nat,
}

type EqNat(a: Nat, b: Nat) {
    .refl(c: Nat): EqNat(c, c),
}

type Empty {}

let zero_eq_one_implies_empty = fun zero_eq_one_implies_empty_(H: EqNat(Nat.o, Nat.s(Nat.o))): Empty {
    match H {
        .refl(_) => impossible,
    }
};

let esoterically_written_identity = fun _(n: Nat): Nat {
    match Nat.s(n) {
        .o => impossible,
        .s(m) => m,
    }
};

let identity_correct = fun _(n: Nat): EqNat(n, esoterically_written_identity(n)) {
    EqNat.refl(n)
};
