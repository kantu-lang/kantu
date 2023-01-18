type U {
    .u: U,
}

let foo = forall(x: U.u) { U };
