mod nat;
pub use nat.*;

mod eq;
pub use eq.*;

pub let(*) plus = fun plus(-a: Nat, b: Nat): Nat {
    match a {
        .O => b,
        .S(a') => Nat.S(plus(a', b)),
    }
};

mod lemmas;
pub use lemmas.*;

pub let plus_comm = fun plus_comm(-a: Nat, b: Nat): Eq(Nat, plus(a, b), plus(b, a)) {
    match a {
        .O =>
            match b {
                .O => Eq.Refl(Nat, Nat.O),
                .S(b') =>
                    match plus_O(b') {
                        .Refl(_Nat, _b) => Eq.Refl(Nat, Nat.S(plus(b', Nat.O))),
                    },
            },
        .S(a') =>
            match b {
                .O =>
                    match plus_O(a') {
                        .Refl(_Nat, _a) => Eq.Refl(Nat, Nat.S(plus(a', Nat.O))),
                    },
                .S(b') =>
                    match plus_S(a', b') {
                        .Refl(_NatSab, _Sab) =>
                            match plus_S(b', a') {
                                .Refl(_NatSba, _Sba) =>
                                    match plus_comm(a', b') {
                                        .Refl(_NatRec, _Rec) => Eq.Refl(Nat, Nat.S(Nat.S(plus(a', b')))),
                                    },
                            },
                    },
            },
    }
};
