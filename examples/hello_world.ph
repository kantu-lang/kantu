type Unit {
    .Trivial: Unit,
}

type List(T: Type) {
    .Nil(T: Type): List(T),
}

let empty = fun empty(l: List(Unit)): List(Unit) {
    match l {
        .Nil(U) => List.Nil(Unit),
    }
};
