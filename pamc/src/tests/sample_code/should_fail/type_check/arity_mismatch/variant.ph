type Empty {}

type Bar {
    .B(T: Type): Bar,
}

let foo = Bar.B(Empty, Empty);
