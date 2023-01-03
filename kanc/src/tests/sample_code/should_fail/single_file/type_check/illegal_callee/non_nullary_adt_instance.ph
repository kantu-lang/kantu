type U {
    .U: U,
}

type Bar {
    .B(x: U): Bar,
}

let foo = Bar.B(U.U)(U.U);
