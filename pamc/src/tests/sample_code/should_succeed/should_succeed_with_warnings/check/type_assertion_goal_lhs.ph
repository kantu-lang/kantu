type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

type Eq(T: Type, a: T, b: T) {
    .Refl(T: Type, c: T): Eq(T, c, c),
}

let type_assertion_goal_lhs = fun _(n: Nat): Nat {
    match n {
        // WARNING
        .O => check (goal: Nat) {
            Nat.O
        },
        // WARNING
        .S(n') => check (goal: ?) {
            Nat.O
        },
    }
};

