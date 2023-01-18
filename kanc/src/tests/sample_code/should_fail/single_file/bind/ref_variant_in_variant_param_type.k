type U(T: Type, t: T) {}

type Bar {
    .d(n: U(Bar, Bar.d)): Bar,
}
