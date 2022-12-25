type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

let symbolically_invalid = fun _(n: Nat): Nat {
    match n {
        // WARNING
        .O => check (goal = this_symbol_doesnt_exist) {
            Nat.O
        },
        // WARNING
        .S(n') => check (goal = fun _(/* name clash with n */ n: Nat): Nat { Nat.O }(n)) {
            Nat.O
        },
    }
};

let illegal_fun_rec = fun _(n: Nat): Nat {
    match n {
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

let type_check_error = fun _(n: Nat): Nat {
    match n {
        // WARNING
        .O => check (goal = Nat.S(Nat)) {
            Nat.O
        },
        .S(n') => Nat.O,
    }
};
