type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

let foo = fun foo(n: Nat, f: forall(T: Type) { T }): Nat {
    f(Nat)
};
