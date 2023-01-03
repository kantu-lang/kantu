mod nat;

type NatList {
    .Nil: NatList,
    .Cons(car: nat.Nat, cdr: NatList): NatList,
}
