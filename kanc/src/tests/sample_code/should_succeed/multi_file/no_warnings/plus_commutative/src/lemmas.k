use super.*;

pub let plus_S = fun plus_S_(-a: Nat, b: Nat): Eq(Nat, Nat.s(plus(a, b)), plus(a, Nat.s(b))) {
    match a {
        .o => Eq.refl(Nat, Nat.s(b)),
        .s(a') =>
            match plus_S_(a', b) {
                .refl(_Nat, _c) => Eq.refl(Nat, Nat.s(Nat.s(plus(a', b)))),
            },
    }
};

pub let plus_O = fun plus_O_(-n: Nat): Eq(Nat, plus(n, Nat.o), n) {
    match n {
        .o => Eq.refl(Nat, Nat.o),
        .s(n') =>
            match plus_O_(n') {
                .refl(_Nat, _n) => Eq.refl(Nat, Nat.s(plus(n', Nat.o))),
            },
    }
};
