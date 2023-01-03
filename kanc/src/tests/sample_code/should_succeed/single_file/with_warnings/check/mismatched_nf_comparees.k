type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

type Eq(T: Type, a: T, b: T) {
    .Refl(T: Type, c: T): Eq(T, c, c),
}

let mismatched_comparees_non_question = fun _(n: Nat): Nat {
    match n {
        // WARNING
        .O => check (goal = Eq(Nat, Nat.O, Nat.O)) {
            Nat.O
        },
        // WARNING
        .S(n') => check (n = Nat.S(n)) {
            Nat.O
        },
    }
};

let mismatched_comparees_question = fun _(m: Nat): Nat {
    match m {
        // WARNING
        .O => check (m = ?) {
            Nat.O
        },
        // WARNING
        .S(m') => check (m = ?) {
            Nat.O
        },
    }
};
