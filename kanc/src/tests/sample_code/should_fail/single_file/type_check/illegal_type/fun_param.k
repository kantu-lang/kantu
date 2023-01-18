type U {
    .u: U,
}

let foo = fun foo_(x: U.u): U {
    U.u
};
