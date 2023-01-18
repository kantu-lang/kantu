mod nat;

type NatList {
    .nil: NatList,
    .cons(car: nat.Nat, cdr: NatList): NatList,
}
