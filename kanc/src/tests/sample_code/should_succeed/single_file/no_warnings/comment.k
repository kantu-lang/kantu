// Single line
type // Single line
// Single line
Nat {
    // Single line
    .o: // Single line
        // Single line
        Nat, // Single line
    .s(
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
        .o => b,
        .s(a_pred) => Nat.s(plus(a_pred, b)),
    }
};

let mult/**//**/ = fun /*inline*/ mult(-a: Nat, b: Nat): Nat {
    match a/*/*/**/*/*/ {
        .o /***/ => Nat.o,
        .s(a_pred) =>
        /*
            /**/ * / /* */ */
            plus(b, mult(a_pred, b)),
    }
};

let square = fun square(a: Nat): Nat { mult(a, a) };

type Eq(T: Type, left: T, right: T) {
    .refl(T: Type, z: T): Eq(T, z, z),
}
