pub type Eq(T: Type, a: T, b: T) {
    .refl(T: Type, t: T): Eq(T, t, t),
}
