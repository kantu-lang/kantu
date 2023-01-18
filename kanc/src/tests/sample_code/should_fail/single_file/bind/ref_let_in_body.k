type U {
    .u: U,
}

let plus = fun plus(a: U, b: U): U { U.u };

let a = plus(a, a);
