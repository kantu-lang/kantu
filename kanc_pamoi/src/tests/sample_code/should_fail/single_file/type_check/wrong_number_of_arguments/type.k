type U {
    u: U,
}

type V(t: U) {}

let foo = V(U.u, U.u);
