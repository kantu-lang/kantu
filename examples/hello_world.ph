type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

type List(T: Type) {
    .Nil(T: Type): List(T),
    .Cons(T: Type, car: T, cdr: List(T)): List(T),
}

let empty = fun empty(l: List(Nat)): List(Nat) {
    match l {
        .Nil(_T) => List.Nil(Nat),
        .Cons(_T, car, cdr) => List.Nil(Nat),
    }
};
