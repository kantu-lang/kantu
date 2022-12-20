type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

type List(T: Type) {
    .Nil(~Item: Type): List(Item),
    .Cons(~Item: Type, ~car: Item, ~cdr: List(Item)): List(Item),
}

let right = List(Nat);
let wrong = List(Item: Nat);
