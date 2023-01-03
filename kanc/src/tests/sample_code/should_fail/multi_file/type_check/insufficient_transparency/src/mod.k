pub type Eq(T: Type, _: T, _: T) {
    .Refl(T: Type, t: T): Eq(T, t, t),
}

pub let identity = fun _(T: Type, t: T): T {
    t
};

pub let(*) identity_eq = fun _(T: Type, t: T): Eq(T, t, identity(T, t)) {
    Eq.Refl(T, t)
};
