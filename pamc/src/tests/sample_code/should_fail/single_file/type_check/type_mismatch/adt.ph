type U1 {
    .U1: U1,
}

type U2 {
    .U2: U2,
}

let foo = fun foo_(u: U1): U1 {
    U2.U2
};
