type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

let foo = fun _(~x: Nat, ~y: Nat): Nat { x };

type Unit { .C: Unit }

let wrong = fun _(_: Unit):
    forall(~y: Nat, ~x: Nat) { Nat }
{
    foo
};
