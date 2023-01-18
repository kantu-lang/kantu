type Nat {
    .o: Nat,
    .s(n: Nat): Nat,
}

type Eq(T: Type, a: T, b: T) {
    .refl(T: Type, c: T): Eq(T, c, c),
}

let mismatched_comparees_non_question = fun _(n: Nat): Nat {
    match n {
        // WARNING
        .o => check (goal = Eq(Nat, Nat.o, Nat.o)) {
            Nat.o
        },
        // WARNING
        .s(n') => check (n = Nat.s(n)) {
            Nat.o
        },
    }
};

let mismatched_comparees_question = fun _(m: Nat): Nat {
    match m {
        // WARNING
        .o => check (m = ?) {
            Nat.o
        },
        // WARNING
        .s(m') => check (m = ?) {
            Nat.o
        },
    }
};
