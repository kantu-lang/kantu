type U {
    .U: U,
}

let foo = fun foo(x: U): U { U.U };

let bar = foo();
