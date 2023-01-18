//! TODO: We'll need to move this to the "succeed _with_ warnings"
//! directory, after we implement misordered arg warnings.

type Nat {
    .o: Nat,
    .s(_: Nat): Nat,
}

type Eq(T: Type, a: T, b: T) {
    .refl(T: Type, t: T): Eq(T, t, t),
}

let _0 = Nat.o;
let _1 = Nat.s(_0);

let equal_despite_misordered_args = fun _(
    f: forall(~a: Nat, ~b: Nat) { Nat },
): Eq(Nat, f(a: _0, b: _1), f(b: _1, a: _0)) {
    Eq.refl(Nat, f(a: _0, b: _1))
};
