mod nat;
use nat.Nat;

mod eq;
pub use eq.Eq;
use eq.identity as ascribe;

pub let OpaqueNat = Nat;
pub let OpaqueO = ascribe(OpaqueNat, Nat.O);
pub let OpaqueS = ascribe(forall(_: OpaqueNat) { OpaqueNat }, Nat.S);

pub let plus = fun plus(-a: OpaqueNat, b: OpaqueNat): OpaqueNat {
    match a {
        .O => b,
        .S(a') => Nat.S(plus(a', b)),
    }
};

pub let plus_S = fun plus_S_(-a: OpaqueNat, b: OpaqueNat): Eq(OpaqueNat, OpaqueS(plus(a, b)), plus(a, OpaqueS(b))) {
    match a {
        .O => Eq.Refl(Nat, Nat.S(b)),
        .S(a') =>
            match plus_S_(a', b) {
                .Refl(_Nat, _c) => Eq.Refl(Nat, Nat.S(Nat.S(plus(a', b)))),
            },
    }
};

pub let plus_O = fun plus_O_(-n: OpaqueNat): Eq(OpaqueNat, plus(n, OpaqueO), n) {
    match n {
        .O => Eq.Refl(Nat, Nat.O),
        .S(n') =>
            match plus_O_(n') {
                .Refl(_Nat, _n) => Eq.Refl(Nat, Nat.S(plus(n', Nat.O))),
            },
    }
};

pub let plus_comm = fun plus_comm(-a: OpaqueNat, b: OpaqueNat): Eq(OpaqueNat, plus(a, b), plus(b, a)) {
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
