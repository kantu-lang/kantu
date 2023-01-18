type Nat {
    .o: Nat,
    .s(n: Nat): Nat,
}

type List(Item~_: Type) {
    .nil(~Item: Type): List(:Item),
    .cons(~Item: Type, ~car: Item, ~cdr: List(:Item)): List(:Item),
}

let right = List(Item: Nat);
let wrong = List(Nat);
