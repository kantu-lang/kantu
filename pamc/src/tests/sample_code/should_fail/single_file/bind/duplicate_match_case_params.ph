type Pair(T: Type, U: Type) {
    .Pair(T: Type, U: Type, t: T, u: U): Pair(T, U),
}

let first = fun first(T: Type, U: Type, p: Pair(T, U)): T {
    match p {
        .Pair(_T, _U, x, x) => x,
    }
};
