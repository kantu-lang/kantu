type U {
    u: U,
}

let foo = fun foo(): U { U.u };
