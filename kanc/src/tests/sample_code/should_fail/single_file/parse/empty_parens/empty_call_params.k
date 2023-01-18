type U {
    .u: U,
}

let foo = fun foo(x: U): U { U.u };

let bar = foo();
