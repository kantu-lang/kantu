type U {
    u: U,
}

type Bar {
    b(x: U): Bar,
}

let foo = Bar.b(U.u)(U.u);
