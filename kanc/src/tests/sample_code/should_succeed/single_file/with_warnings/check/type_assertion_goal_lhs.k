type Nat {
    .o: Nat,
    .s(n: Nat): Nat,
}

type Eq(T: Type, a: T, b: T) {
    .refl(T: Type, c: T): Eq(T, c, c),
}

let type_assertion_goal_lhs = fun _(n: Nat): Nat {
    match n {
        // WARNING
        .o => check (goal: Nat) {
            Nat.o
        },
        // WARNING
        .s(n') => check (goal: ?) {
            Nat.o
        },
    }
};

