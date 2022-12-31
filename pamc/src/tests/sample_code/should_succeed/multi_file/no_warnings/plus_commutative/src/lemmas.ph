use super.*;

pub let plus_S = fun plus_S_(-a: Nat, b: Nat): Eq(Nat, Nat.S(plus(a, b)), plus(a, Nat.S(b))) {
    match a {
        .O => Eq.Refl(Nat, Nat.S(b)),
        .S(a') =>
            match plus_S_(a', b) {
                .Refl(_Nat, _c) => Eq.Refl(Nat, Nat.S(Nat.S(plus(a', b)))),
            },
    }
};

pub let plus_O = fun plus_O_(-n: Nat): Eq(Nat, plus(n, Nat.O), n) {
    match n {
        .O => Eq.Refl(Nat, Nat.O),
        .S(n') =>
            match plus_O_(n') {
                .Refl(_Nat, _n) => Eq.Refl(Nat, Nat.S(plus(n', Nat.O))),
            },
    }
};
