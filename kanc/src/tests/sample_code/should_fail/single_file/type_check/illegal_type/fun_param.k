type U {
    .U: U,
}

let foo = fun foo_(x: U.U): U {
    U.U
};
