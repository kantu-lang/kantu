type NatList {
    .nil: NatList,
    .cons(car: pack.math.Nat, cdr: NatList): NatList,
}

let list_to_nat_list = fun list_to_nat_list(
    -nats: mod.list.List(pack.math.Nat),
): NatList {
    match nats {
        .nil => mod.NatList.nil,
        .cons(_Nat, car, cdr) => NatList.cons(
            car,
            list_to_nat_list(cdr),
        ),
    }
};

let Nat = super4.Nat;
