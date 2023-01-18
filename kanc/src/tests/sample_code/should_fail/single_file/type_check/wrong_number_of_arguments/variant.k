type Empty {}

type Bar {
    .b(T: Type): Bar,
}

let foo = Bar.b(Empty, Empty);
