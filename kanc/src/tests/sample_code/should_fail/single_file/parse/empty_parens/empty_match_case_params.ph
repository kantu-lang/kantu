type U {
    .T: U,
}

let foo = match U.T {
    .T() => U.T,
};
