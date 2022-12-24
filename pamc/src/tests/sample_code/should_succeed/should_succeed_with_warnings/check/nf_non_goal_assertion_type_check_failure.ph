type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

let symbolically_invalid = fun _(n: Nat): Nat {
    match Nat.O {
        // WARNING
        .O => check (n = this_symbol_doesnt_exist) {
            Nat.O
        },
        // WARNING
        .S(n') => check (n = fun _(/* name clash with n */ n: Nat): Nat { Nat.O }(n)) {
            Nat.O
        },
    }
};

let illegal_fun_rec = fun _(n: Nat): Nat {
    match Nat.O {
        .O =>
        check (
            // WARNING
            n =
                fun infinite_loop(-decreasing: Nat): Nat {
                    infinite_loop(decreasing)
                }(n)
        ) {
            Nat.O
        },
        .S(n') =>
        check (
            // WARNING
            n =
                fun infinite_loop(not_decreasing: Nat): Nat {
                    infinite_loop(not_decreasing)
                }(n)
        ) {
            Nat.O
        },
    }
};

let type_check_error = fun _(n: Nat): Nat {
    match Nat.O {
        // WARNING
        .O => check (n = Nat.S(Nat)) {
            Nat.O
        },
        // WARNING
        .S(n') => check (Nat.S(Nat) = n) {
            Nat.O
        },
    }
};

let type_check_error2 = fun _(n: Nat): Nat {
    match Nat.O {
        // WARNING
        .O => check (Nat.S(Nat) = Nat.S(Nat)) {
            Nat.O
        },
        .S(n') => Nat.O,
    }
};
