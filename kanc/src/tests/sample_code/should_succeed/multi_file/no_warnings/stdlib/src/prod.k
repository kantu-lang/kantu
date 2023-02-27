pub type Prod(T: Type, U: Type) {
    pair(T: Type, t: T, U: Type, u: U): Prod(T, U),
}

pub let(*) first = fun _(T: Type, U: Type, p: Prod(T, U)): T {
    match p {
        pair(_, t, _, _) => t,
    }
};

pub let(*) second = fun _(T: Type, U: Type, p: Prod(T, U)): U {
    match p {
        pair(_, _, _, u) => u,
    }
};
