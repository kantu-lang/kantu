type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

type List(T: Type) {
    .Nil(T: Type): List(T),
    .Cons(T: Type, car: T, cdr: List(T)): List(T),
}

let plus = fun plus(-a: Nat, b: Nat): Nat {
    match a {
        .O => b,
        .S(a_pred) => Nat.S(plus(a_pred, b)),
    }
};
