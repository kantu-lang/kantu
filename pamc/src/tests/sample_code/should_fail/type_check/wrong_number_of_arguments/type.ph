type U {
    .U: U,
}

type V(t: U) {}

let foo = V(U.U, U.U);
