use pack.Empty;

pub type Eq(T: Type, a: T, b: T) {
    .refl(T: Type, t: T): Eq(T, t, t),
}

pub let(*) Neq = fun _(T: Type, a: T, b: T): Type {
    forall(_: Eq(T, a, b)) { Empty }
};
