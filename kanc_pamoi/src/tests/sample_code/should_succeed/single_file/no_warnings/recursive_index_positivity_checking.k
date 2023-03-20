type List(T: Type) {
    nil(T: Type): List(T),
    cons(T: Type, car: T, cdr: List(T)): List(T),
}

type Goofy {
    c(_: List(Goofy)): Goofy,
}

let goofy = Goofy.c(List.nil(Goofy));
