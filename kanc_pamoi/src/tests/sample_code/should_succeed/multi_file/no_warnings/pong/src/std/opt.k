pub type Opt(T: Type) {
    none(T: Type): Opt(T),
    some(T: Type, t: T): Opt(T),
}
