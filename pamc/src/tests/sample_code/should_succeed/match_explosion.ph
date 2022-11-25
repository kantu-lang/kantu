type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

type EqNat(a: Nat, b: Nat) {
    .Refl(c: Nat): EqNat(c, c),
}

type Empty {}

type DummyType {
    .WeShouldHaveExplodedByNow: DummyType,
}

let zero_eq_one_implies_empty = fun zero_eq_one_implies_empty_(H: EqNat(Nat.O, Nat.S(Nat.O))): Empty {
    match H {
        .Refl(_c) => DummyType.WeShouldHaveExplodedByNow,
    }
};
