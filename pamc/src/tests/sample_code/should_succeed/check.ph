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
            check goal: Eq(U, c, c) {
                Eq.Refl(U, c)
            },
    }
};

let foo = fun _(n: Nat): Nat {
    match n {
        .O => check n: Nat = Nat.O {
            Nat.O
        },
        .S(n') => check n: Nat = Nat.S(n') {
            Nat.O
        },
    }
};

let m = Nat.O;
let expression_checkee_matrix = match m {
    .O => check m: ? = ? {
        Nat.O
    },
    .S(n') => check m: Nat = ? {
        Nat.O
    },
};
let expression_checkee_matrix2 = match m {
    .O => check m: ? = Nat.O {
        Nat.O
    },
    .S(n') => check m: Nat = Nat.S(n') {
        Nat.O
    },
};

let goal_checkee = fun _(n: Nat): Nat {
    match Nat.O {
        .O => check goal: ? {
            Nat.O
        },
        .S(n') => check goal: Nat {
            Nat.O
        },
    }
};
