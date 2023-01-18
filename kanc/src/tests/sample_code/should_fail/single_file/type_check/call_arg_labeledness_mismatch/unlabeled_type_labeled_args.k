type Nat {
    .o: Nat,
    .s(n: Nat): Nat,
}

type List(T: Type) {
    .nil(~Item: Type): List(Item),
    .cons(~Item: Type, ~car: Item, ~cdr: List(Item)): List(Item),
}

let right = List(Nat);
let wrong = List(Item: Nat);
