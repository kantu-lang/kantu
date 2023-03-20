type U {
    u: U,
}

let f = fun g(g: U): U {
    U.u
};
