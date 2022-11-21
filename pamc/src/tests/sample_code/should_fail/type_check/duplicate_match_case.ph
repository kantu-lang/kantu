type U {
    .U: U,
}

let foo = match U.U {
    .U => U.U,
    .U => U.U,
};
