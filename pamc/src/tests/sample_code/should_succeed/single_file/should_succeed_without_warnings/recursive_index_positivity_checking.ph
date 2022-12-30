type List(T: Type) {
    .Nil(T: Type): List(T),
    .Cons(T: Type, car: T, cdr: List(T)): List(T),
}

type Goofy {
    .C(_: List(Goofy)): Goofy,
}

let goofy = Goofy.C(List.Nil(Goofy));
