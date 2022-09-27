type U(T: Type, t: T) {}

type Bar {
    .D(n: U(Bar, Bar.D)): Bar,
}
