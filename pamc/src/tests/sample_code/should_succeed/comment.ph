// Single line
type // Single line
// Single line
Nat {
    // Single line
    .O: // Single line
        // Single line
        Nat, // Single line
    .S(
        // Single line
        n: // Single line
            // Single line
            Nat // Single line
    ): // Single line
        Nat // Single line
    // Single line
    ,
}

let /* multi
line
comment*/plus = fun/*inline*/plus(- /*inline*/ a: /* inline */ Nat, b: Nat): Nat {
    match a /* nested /* nested */ */ {
        .O => b,
        .S(a_pred) => Nat.S(plus(a_pred, b)),
    }
};

let mult/**//**/ = fun /*inline*/ mult(-a: Nat, b: Nat): Nat {
    match a/*/**/*/ {
        .O => Nat.O,
        .S(a_pred) => plus(b, mult(a_pred, b)),
    }
};

let square = fun square(a: Nat): Nat { mult(a, a) };

type Eq(T: Type, left: T, right: T) {
    .Refl(T: Type, z: T): Eq(T, z, z),
}
