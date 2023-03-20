pub type Eq(T: Type, left: T, right: T) {
    refl(T: Type, z: T): Eq(T, z, z),
}

pub let(*) identity = fun _(T: Type, t: T): T {
    t
};
