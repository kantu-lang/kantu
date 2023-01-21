type U {
    u: U,
}

let foo = fun bar_(x: U): U {
    U.u
}(U.u, U.u);
