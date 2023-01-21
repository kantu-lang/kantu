type Nat {
    o: Nat,
    s(n: Nat): Nat,
}

let symbolically_invalid = fun _(n: Nat): Nat {
    match n {
        // WARNING
        o => check (goal = this_symbol_doesnt_exist) {
            Nat.o
        },
        // WARNING
        s(n') => check (goal = fun _(/* name clash with n */ n: Nat): Nat { Nat.o }(n)) {
            Nat.o
        },
    }
};

let illegal_fun_rec = fun _(n: Nat): Nat {
    match n {
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

let type_check_error = fun _(n: Nat): Nat {
    match n {
        // WARNING
        o => check (goal = Nat.s(Nat)) {
            Nat.o
        },
        s(n') => Nat.o,
    }
};
