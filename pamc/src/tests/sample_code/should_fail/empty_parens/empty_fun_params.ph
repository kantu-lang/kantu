type U {
    .U: U,
}

let foo = fun foo(): U { U.U };
