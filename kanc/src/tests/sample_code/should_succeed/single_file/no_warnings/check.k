type Nat {
    .o: Nat,
    .s(n: Nat): Nat,
}

type Eq(T: Type, a: T, b: T) {
    .refl(T: Type, c: T): Eq(T, c, c),
}

let eq_comm = fun _(T: Type, a: T, b: T, H: Eq(T, a, b)): Eq(T, b, a) {
    match H {
        .refl(U, c) =>
            check (goal = Eq(U, c, c)) {
                Eq.refl(U, c)
            },
    }
};

let foo = fun _(n: Nat): Nat {
    match n {
        .o => check (n: Nat, n = Nat.o) {
            Nat.o
        },
        .s(n') => check (n: Nat, n = Nat.s(n')) {
            Nat.o
        },
    }
};

let goal_checkee = fun _(n: Nat): Nat {
    match n {
        .o => check (goal = Nat) {
            Nat.o
        },
        .s(n') => check (goal = Nat) {
            Nat.o
        },
    }
};
