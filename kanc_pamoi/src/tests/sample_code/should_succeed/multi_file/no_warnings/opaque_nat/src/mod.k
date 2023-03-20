mod nat;
use nat.Nat;

mod eq;
pub use eq.Eq;
use eq.identity as ascribe;

pub let OpaqueNat = Nat;
pub let OpaqueO = ascribe(OpaqueNat, Nat.o);
pub let OpaqueS = ascribe(forall(_: OpaqueNat) { OpaqueNat }, Nat.s);

pub let plus = fun plus(-a: OpaqueNat, b: OpaqueNat): OpaqueNat {
    match a {
        o => b,
        s(a') => Nat.s(plus(a', b)),
    }
};

pub let plus_S = fun plus_S_(-a: OpaqueNat, b: OpaqueNat): Eq(OpaqueNat, OpaqueS(plus(a, b)), plus(a, OpaqueS(b))) {
    match a {
        o => Eq.refl(Nat, Nat.s(b)),
        s(a') =>
            match plus_S_(a', b) {
                refl(_Nat, _c) => Eq.refl(Nat, Nat.s(Nat.s(plus(a', b)))),
            },
    }
};

pub let plus_O = fun plus_O_(-n: OpaqueNat): Eq(OpaqueNat, plus(n, OpaqueO), n) {
    match n {
        o => Eq.refl(Nat, Nat.o),
        s(n') =>
            match plus_O_(n') {
                refl(_Nat, _n) => Eq.refl(Nat, Nat.s(plus(n', Nat.o))),
            },
    }
};

pub let plus_comm = fun plus_comm(-a: OpaqueNat, b: OpaqueNat): Eq(OpaqueNat, plus(a, b), plus(b, a)) {
    match a {
        o =>
            match b {
                o => Eq.refl(Nat, Nat.o),
                s(b') =>
                    match plus_O(b') {
                        refl(_Nat, _b) => Eq.refl(Nat, Nat.s(plus(b', Nat.o))),
                    },
            },
        s(a') =>
            match b {
                o =>
                    match plus_O(a') {
                        refl(_Nat, _a) => Eq.refl(Nat, Nat.s(plus(a', Nat.o))),
                    },
                s(b') =>
                    match plus_S(a', b') {
                        refl(_NatSab, _Sab) =>
                            match plus_S(b', a') {
                                refl(_NatSba, _Sba) =>
                                    match plus_comm(a', b') {
                                        refl(_NatRec, _Rec) => Eq.refl(Nat, Nat.s(Nat.s(plus(a', b')))),
                                    },
                            },
                    },
            },
    }
};
