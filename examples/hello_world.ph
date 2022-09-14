type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

type List(T: Type) {
    .Nil(T: Type): List(T),
    .Cons(T: Type, car: T, cdr: List(T)): List(T),
}

let plus = fun plus(a: Nat, b: Nat): Nat => match a {
    .O => b,
    .S(a_pred) => Nat.S(plus(a_pred, b)),
};

let mult = fun mult(a: Nat, b: Nat): Nat => match a {
    .O => Nat.O,
    .S(a_pred) => plus(b, mult(a_pred, b)),
};

let square = fun square(a: Nat): Nat => mult(a, a);

let map = fun map(T: Type, U: Type, l: List(T), f: forall(v: T) => U): List(U) => match l {
    .Nil(_T) => List.Nil(U),
    .Cons(_T, car, cdr) => List.Cons(U, f(car), map(T, U, cdr, f)),
};

let square_all = fun square_all(l: List(Nat)): List(Nat) => map(Nat, Nat, l, square);
