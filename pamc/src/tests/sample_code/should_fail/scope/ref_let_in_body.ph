type U {
    .U: U,
}

let plus = fun plus(a: U, b: U): U { U.U };

let a = plus(a, a);
