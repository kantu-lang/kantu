pub type Eq(T: Type, _: T, _: T) {
    .refl(T: Type, t: T): Eq(T, t, t),
}

pub let(*) identity = fun _(T: Type, t: T): T {
    t
};

pub let(*) identity_eq = fun _(T: Type, t: T): Eq(T, t, identity(T, t)) {
    Eq.refl(T, t)
};
