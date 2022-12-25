type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

type EqNat(a: Nat, b: Nat) {
    .Refl(c: Nat): EqNat(c, c),
}

type Empty {}

let zero_eq_one_implies_empty = fun zero_eq_one_implies_empty_(H: EqNat(Nat.O, Nat.S(Nat.O))): Empty {
    match H {
        .Refl(_) => impossible,
    }
};

let esoterically_written_identity = fun _(n: Nat): Nat {
    match Nat.S(n) {
        .O => impossible,
        .S(m) => m,
    }
};

let identity_correct = fun _(n: Nat): EqNat(n, esoterically_written_identity(n)) {
    EqNat.Refl(n)
};
