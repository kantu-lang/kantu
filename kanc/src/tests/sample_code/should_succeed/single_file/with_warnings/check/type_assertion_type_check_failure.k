type Nat {
    .o: Nat,
    .s(n: Nat): Nat,
}

let symbolically_invalid = fun _(n: Nat): Nat {
    match n {
        // WARNING
        .o => check (n: this_symbol_doesnt_exist) {
            Nat.o
        },
        // WARNING
        .s(n') => check (n: fun _(/* name clash with n */ n: Nat): Type { Nat }(n)) {
            Nat.o
        },
    }
};

let illegal_fun_rec = fun _(n: Nat): Nat {
    match n {
        .o =>
        check (
            // WARNING
            n:
                fun infinite_loop(-decreasing: Nat): Type {
                    infinite_loop(decreasing)
                }(n)
        ) {
            Nat.o
        },
        .s(n') =>
        check (
            // WARNING
            n:
                fun infinite_loop(not_decreasing: Nat): Type {
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
        .o => check (n: Nat.s(Nat)) {
            Nat.o
        },
        // WARNING
        .s(n') => check (Nat.s(Nat): n) {
            Nat.o
        },
    }
};

let type_check_error2 = fun _(n: Nat): Nat {
    match n {
        .o => check (
            // WARNING
            Nat.s(Nat):
                // WARNING
                Nat.s(Nat)
        ) {
            Nat.o
        },
        .s(n') => Nat.o,
    }
};
