pub type Option(T: Type) {
    .none(T: Type): Option(T),
    .some(T: Type, t: T): Option(T),
}
