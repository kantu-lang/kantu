type U {
    t: U,
}

let foo = match U.t {
    t() => U.t,
};
