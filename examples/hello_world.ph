type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

type Foo(T: Type) {
    .Bar: Foo(Nat),
}
