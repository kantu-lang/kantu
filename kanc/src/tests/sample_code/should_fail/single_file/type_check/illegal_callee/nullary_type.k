type U {
    u: U,
}

let foo = U(U.u);
