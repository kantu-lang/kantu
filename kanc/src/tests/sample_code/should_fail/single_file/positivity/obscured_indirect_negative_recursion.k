type Empty {}

type Dummy {
    c: Dummy,
}

type Bool {
    true: Bool,
    false: Bool,
}

type Eq(T: Type, x: T, y: T) {
    refl(T: Type, z: T): Eq(T, z, z),
}

type Not(T: Type) {
    not_c(
        b: Bool,
        H: Eq(Bool, Bool.true, b),
        B: Type,
        f: forall(_: B) { Empty }
    ): Not(match b { true => B, false => Dummy }),
}

type Bad {
    c(n: Not(Bad)): Bad,
}

let identity = fun _(T: Type, t: T): T {
    t
};

let ascribe = identity;

type TypeEq(A: Type, B: Type) {
    refl(C: Type): TypeEq(C, C),
}

let not_bad = fun _(b: Bad): Empty {
    match b {
        c(n) =>
            match n {
                not_c(bool, H, B, f) =>
                    match H {
                        refl(_Bool, _True) =>
                            check (
                                n: Not(Bad),
                                n: Not(B),
                            ) {
                                match ascribe(TypeEq(Not(Bad), Not(B)), TypeEq.refl(Not(Bad))) {
                                    refl(_NotBad_NotB) =>
                                        check (B = Bad) {
                                            ascribe(forall(_: Bad) { Empty }, f)(b)
                                        },
                                }
                            },
                    },
            },
    }
};
