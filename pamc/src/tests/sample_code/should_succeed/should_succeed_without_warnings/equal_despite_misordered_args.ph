//! TODO: We'll need to move this to the "succeed _with_ warnings"
//! directory, after we implement misordered arg warnings.

type Nat {
    .O: Nat,
    .S(_: Nat): Nat,
}

type Eq(T: Type, a: T, b: T) {
    .Refl(T: Type, t: T): Eq(T, t, t),
}

let _0 = Nat.O;
let _1 = Nat.S(_0);

let equal_despite_misordered_args = fun _(
    f: forall(~a: Nat, ~b: Nat) { Nat },
): Eq(Nat, f(a: _0, b: _1), f(b: _1, a: _0)) {
    Eq.Refl(Nat, f(a: _0, b: _1))
};
