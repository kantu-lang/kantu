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


let mismatched_comparees = fun _(n: Nat): Nat {
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

let m = Nat.O;
let question_mark_rhs = match m {
    // WARNING (x2)
    .O => check (m: ?, m = ?) {
        Nat.O
    },
    // WARNING (x2)
    .S(m') => check (m: ?, m = ?) {
        Nat.O
    },
};

let goal_checkee = fun _(n: Nat): Nat {
    match Nat.O {
        .O => check (goal = Nat) {
            Nat.O
        },
        // WARNING
        .S(n') => check (goal = ?) {
            Nat.O
        },
    }
};

let symbolically_invalid_annotations1 = fun _(n: Nat): Nat {
    match Nat.O {
        // WARNING
        .O => check (goal = this_symbol_doesnt_exist) {
            Nat.O
        },
        // WARNING
        .S(n') => check (goal = fun _(n: name_clash_with_n_Nat): Type { Nat }(n)) {
            Nat.O
        },
    }
};

let symbolically_invalid_annotations2 = fun _(n: Nat): Nat {
    match n {
        // WARNING (x2)
        .O => check (n: this_symbol_doesnt_exist, n = neither_does_this_one) {
            Nat.O
        },
        // WARNING
        .S(n') => check (goal = fun _(m: Nat, error: NonexistentFoo): Type { Nat }(n, n)) {
            Nat.O
        },
    }
};

let symbolically_invalid_annotations3 = fun _(n: Nat): Nat {
    match Nat.O {
        // WARNING
        .O => check (goal = fun _(p: Nat, q: Nat): Type { not_defined }(n, n)) {
            Nat.O
        },
        // WARNING
        .S(n') => check (goal = fun _(n: name_clash_with_n_Nat): Type { Nat }(n)) {
            Nat.O
        },
    }
};

let illegal_fun_rec_annotations1 = fun _(n: Nat): Nat {
    match Nat.O {
        .O =>
        check (
            // WARNING
            goal =
                fun infinite_loop(-decreasing: Nat): Nat {
                    infinite_loop(decreasing)
                }(n)
        ) {
            Nat.O
        },
        .S(n') =>
        check (
            // WARNING
            goal =
                fun infinite_loop(not_decreasing: Nat): Nat {
                    infinite_loop(not_decreasing)
                }(n)
        ) {
            Nat.O
        },
    }
};

let illegal_fun_rec_annotations2 = fun recursive_identity(-n: Nat): Nat {
    match n {
        .O =>
        check (
            // WARNING
            goal =
                fun _(
                    z: Nat,
                    y:
                        fun infinite_loop(-decreasing: Nat): Type {
                            infinite_loop(decreasing)
                        }(z),
                ): Type {
                    y
                }(n, Nat)
        ) {
            Nat.O
        },
        
        .S(n') =>
        check (
            // WARNING
            goal =
                fun _(
                    z: Nat,
                    y:
                        fun infinite_loop(-decreasing: Nat): Type {
                            infinite_loop(decreasing)
                        }(z),
                ): Type {
                    y
                }(n, Nat)
        ) {
            recursive_identity(n')
        },
    }
};
