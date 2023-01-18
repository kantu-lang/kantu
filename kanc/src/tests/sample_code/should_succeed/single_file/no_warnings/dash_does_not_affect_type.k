type Nat {
    .o: Nat,
    .s(n: Nat): Nat,
}

let foo = fun _(-a: Nat, b: Nat): Nat { a };

type Unit { .c: Unit }

let ok = fun _(_: Unit):
    forall(x: Nat, y: Nat) { Nat }
{
    foo
};
