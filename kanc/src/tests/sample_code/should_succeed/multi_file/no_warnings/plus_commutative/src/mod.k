mod nat;
pub use nat.*;

mod eq;
pub use eq.*;

pub let(*) plus = fun plus(-a: Nat, b: Nat): Nat {
    match a {
        .o => b,
        .s(a') => Nat.s(plus(a', b)),
    }
};

mod lemmas;
pub use lemmas.*;

pub let plus_comm = fun plus_comm(-a: Nat, b: Nat): Eq(Nat, plus(a, b), plus(b, a)) {
    match a {
        .o =>
            match b {
                .o => Eq.refl(Nat, Nat.o),
                .s(b') =>
                    match plus_O(b') {
                        .refl(_Nat, _b) => Eq.refl(Nat, Nat.s(plus(b', Nat.o))),
                    },
            },
        .s(a') =>
            match b {
                .o =>
                    match plus_O(a') {
                        .refl(_Nat, _a) => Eq.refl(Nat, Nat.s(plus(a', Nat.o))),
                    },
                .s(b') =>
                    match plus_S(a', b') {
                        .refl(_NatSab, _Sab) =>
                            match plus_S(b', a') {
                                .refl(_NatSba, _Sba) =>
                                    match plus_comm(a', b') {
                                        .refl(_NatRec, _Rec) => Eq.refl(Nat, Nat.s(Nat.s(plus(a', b')))),
                                    },
                            },
                    },
            },
    }
};
