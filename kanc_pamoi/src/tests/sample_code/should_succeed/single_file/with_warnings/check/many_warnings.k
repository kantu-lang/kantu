type Nat {
    o: Nat,
    s(n: Nat): Nat,
}

type Eq(T: Type, a: T, b: T) {
    refl(T: Type, c: T): Eq(T, c, c),
}

let eq_comm = fun _(T: Type, a: T, b: T, H: Eq(T, a, b)): Eq(T, b, a) {
    match H {
        refl(U, c) =>
            check (goal = Eq(U, c, c)) {
                Eq.refl(U, c)
            },
    }
};

let foo = fun _(n: Nat): Nat {
    match n {
        o => check (n: Nat, n = Nat.o) {
            Nat.o
        },
        s(n') => check (n: Nat, n = Nat.s(n')) {
            Nat.o
        },
    }
};

let type_assertion_goal_lhs = fun _(n: Nat): Nat {
    match n {
        // WARNING
        o => check (goal: Nat) {
            Nat.o
        },
        // WARNING
        s(n') => check (goal: ?) {
            Nat.o
        },
    }
};


let mismatched_comparees = fun _(n: Nat): Nat {
    match n {
        // WARNING
        o => check (goal = Eq(Nat, Nat.o, Nat.o)) {
            Nat.o
        },
        // WARNING
        s(n') => check (n = Nat.s(n)) {
            Nat.o
        },
    }
};

let m = Nat.o;
let question_mark_rhs = match m {
    // WARNING (x2)
    o => check (m: ?, m = ?) {
        Nat.o
    },
    // WARNING (x2)
    s(m') => check (m: ?, m = ?) {
        Nat.o
    },
};

let goal_checkee = fun _(n: Nat): Nat {
    match Nat.o {
        o => check (goal = Nat) {
            Nat.o
        },
        // WARNING
        s(n') => check (goal = ?) {
            Nat.o
        },
    }
};

let symbolically_invalid_annotations1 = fun _(n: Nat): Nat {
    match Nat.o {
        // WARNING
        o => check (goal = this_symbol_doesnt_exist) {
            Nat.o
        },
        // WARNING
        s(n') => check (goal = fun _(n: name_clash_with_n_Nat): Type { Nat }(n)) {
            Nat.o
        },
    }
};

let symbolically_invalid_annotations2 = fun _(n: Nat): Nat {
    match n {
        // WARNING (x2)
        o => check (n: this_symbol_doesnt_exist, n = neither_does_this_one) {
            Nat.o
        },
        // WARNING
        s(n') => check (goal = fun _(m: Nat, error: NonexistentFoo): Type { Nat }(n, n)) {
            Nat.o
        },
    }
};

let symbolically_invalid_annotations3 = fun _(n: Nat): Nat {
    match Nat.o {
        // WARNING
        o => check (goal = fun _(p: Nat, q: Nat): Type { not_defined }(n, n)) {
            Nat.o
        },
        // WARNING
        s(n') => check (goal = fun _(n: name_clash_with_n_Nat): Type { Nat }(n)) {
            Nat.o
        },
    }
};

let illegal_fun_rec_annotations1 = fun _(n: Nat): Nat {
    match Nat.o {
        o =>
        check (
            // WARNING
            goal =
                fun infinite_loop(-decreasing: Nat): Nat {
                    infinite_loop(decreasing)
                }(n)
        ) {
            Nat.o
        },
        s(n') =>
        check (
            // WARNING
            goal =
                fun infinite_loop(not_decreasing: Nat): Nat {
                    infinite_loop(not_decreasing)
                }(n)
        ) {
            Nat.o
        },
    }
};

let illegal_fun_rec_annotations2 = fun recursive_identity(-n: Nat): Nat {
    match n {
        o =>
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
            Nat.o
        },
        
        s(n') =>
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
