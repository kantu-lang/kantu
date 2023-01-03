type U {
    .U: U,
}

let foo = U(U.U);
