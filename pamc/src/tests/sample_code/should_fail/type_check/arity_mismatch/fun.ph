type U {
    .U: U,
}

let foo = fun bar_(x: U): U {
    U.U
}(U.U, U.U);
