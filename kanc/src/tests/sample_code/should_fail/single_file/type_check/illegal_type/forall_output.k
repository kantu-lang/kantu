type U {
    .u: U,
}

let foo = forall(x: U) { U.u };
