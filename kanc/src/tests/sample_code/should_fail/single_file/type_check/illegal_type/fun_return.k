type U {
    .u: U,
}

let foo = fun foo_(x: U): U.u {
    U.u
};
