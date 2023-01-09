pub type List(T: Type) {
    .nil(T: Type): List(T),
    .cons(T: Type, car: T, cdr: List(T)): List(T),
}
