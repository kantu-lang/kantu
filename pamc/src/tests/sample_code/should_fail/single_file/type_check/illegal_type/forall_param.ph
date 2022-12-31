type U {
    .U: U,
}

let foo = forall(x: U.U) { U };
