type EqType(_: Type, _: Type) {
    refl(T: Type): EqType(T, T),
}

type False {}

type Bad {
    bad(T: Type, H: EqType(T, Bad), not: forall(_: T) { False }): Bad,
}

let not_bad = fun _(b: Bad): False {
    match b {
        bad(T, H, not) => match H {
            refl(_) => not(b),
        },
    }
};

let bad = Bad.bad(Bad, EqType.refl(Bad), not_bad);
let false = not_bad(bad);
