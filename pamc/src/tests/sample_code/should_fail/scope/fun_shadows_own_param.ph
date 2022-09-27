type U {
    .U: U,
}

let f = fun g(g: U): U {
    U.U
};
