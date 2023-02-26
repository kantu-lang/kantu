pub type Pair(T: Type, U: Type) {
    pair(T: Type, t: T, U: Type, u: U): Pair(T, U),
}

pub let(*) first = fun _(T: Type, U: Type, p: Pair(T, U)): T {
    match p {
        pair(_, t, _, _) => t,
    }
};

pub let(*) second = fun _(T: Type, U: Type, p: Pair(T, U)): U {
    match p {
        pair(_, _, _, u) => u,
    }
};
