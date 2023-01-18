type U {
    .u: U,
}

let foo = U.u(U.u);
