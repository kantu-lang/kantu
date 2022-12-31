type Empty {}

type Dummy {
    .C: Dummy,
}

type Bool {
    .True: Bool,
    .False: Bool,
}

type Eq(T: Type, x: T, y: T) {
    .Refl(T: Type, z: T): Eq(T, z, z),
}

type Not(T: Type) {
    .NotC(
        b: Bool,
        H: Eq(Bool, Bool.True, b),
        B: Type,
        f: forall(_: B) { Empty }
    ): Not(match b { .True => B, .False => Dummy }),
}

type Bad {
    .C(n: Not(Bad)): Bad,
}

let identity = fun _(T: Type, t: T): T {
    t
};

let ascribe = identity;

type TypeEq(A: Type, B: Type) {
    .Refl(C: Type): TypeEq(C, C),
}

let not_bad = fun _(b: Bad): Empty {
    match b {
        .C(n) =>
            match n {
                .NotC(bool, H, B, f) =>
                    match H {
                        .Refl(_Bool, _True) =>
                            check (
                                n: Not(Bad),
                                n: Not(B),
                            ) {
                                match ascribe(TypeEq(Not(Bad), Not(B)), TypeEq.Refl(Not(Bad))) {
                                    .Refl(_NotBad_NotB) =>
                                        check (B = Bad) {
                                            ascribe(forall(_: Bad) { Empty }, f)(b)
                                        },
                                }
                            },
                    },
            },
    }
};
