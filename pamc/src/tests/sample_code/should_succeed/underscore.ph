type List(_: Type) {
    .Nil(T: Type): List(T),
    .Cons(T: Type, car: T, cdr: List(T)): List(T),
}
