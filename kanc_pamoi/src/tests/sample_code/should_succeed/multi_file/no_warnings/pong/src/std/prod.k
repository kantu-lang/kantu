pub type Prod(L: Type, R: Type) {
    pair(L: Type, l: L, R: Type, r: R): Prod(L, R),
}

pub let first = fun _(L: Type, R: Type, p: Prod(L, R)): L {
    match p {
        pair(_, l, _, _) => l,
    }
};

pub let second = fun _(L: Type, R: Type, p: Prod(L, R)): R {
    match p {
        pair(_, _, _, r) => r,
    }
};
