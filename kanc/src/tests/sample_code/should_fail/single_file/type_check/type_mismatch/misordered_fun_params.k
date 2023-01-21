type Nat {
    o: Nat,
    s(n: Nat): Nat,
}

let foo = fun _(~x: Nat, ~y: Nat): Nat { x };

type Unit { c: Unit }

let wrong = fun _(_: Unit):
    forall(~y: Nat, ~x: Nat) { Nat }
{
    foo
};
