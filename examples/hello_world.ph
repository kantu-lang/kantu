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

let mult = fun mult(-a: Nat, b: Nat): Nat {
    match a {
        .O => Nat.O,
        .S(a_pred) => plus(b, mult(a_pred, b)),
    }
};

let square = fun square(a: Nat): Nat { mult(a, a) };

let map = fun map(A: Type, B: Type, -l: List(A), f: forall(v: A) { B }): List(B) {
    match l {
        .Nil(_A1) => List.Nil(B),
        .Cons(_A2, car, cdr) => List.Cons(B, f(car), map(A, B, cdr, f)),
    }
};
