type NatList {
    .Nil: NatList,
    .Cons(car: pack.math.Nat, cdr: NatList): NatList,
}

let list_to_nat_list = fun list_to_nat_list(
    -nats: mod.list.List(pack.math.Nat),
): NatList {
    match nats {
        .Nil => mod.NatList.Nil,
        .Cons(_Nat, car, cdr) => NatList.Cons(
            car,
            list_to_nat_list(cdr),
        ),
    }
};

let Nat = super4.Nat;
