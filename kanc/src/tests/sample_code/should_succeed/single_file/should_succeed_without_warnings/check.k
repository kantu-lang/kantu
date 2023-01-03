type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

type Eq(T: Type, a: T, b: T) {
    .Refl(T: Type, c: T): Eq(T, c, c),
}

let eq_comm = fun _(T: Type, a: T, b: T, H: Eq(T, a, b)): Eq(T, b, a) {
    match H {
        .Refl(U, c) =>
            check (goal = Eq(U, c, c)) {
                Eq.Refl(U, c)
            },
    }
};

let foo = fun _(n: Nat): Nat {
    match n {
        .O => check (n: Nat, n = Nat.O) {
            Nat.O
        },
        .S(n') => check (n: Nat, n = Nat.S(n')) {
            Nat.O
        },
    }
};

let goal_checkee = fun _(n: Nat): Nat {
    match n {
        .O => check (goal = Nat) {
            Nat.O
        },
        .S(n') => check (goal = Nat) {
            Nat.O
        },
    }
};
