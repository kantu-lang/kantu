type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

type List(Item~_: Type) {
    .Nil(~Item: Type): List(:Item),
    .Cons(~Item: Type, ~car: Item, ~cdr: List(:Item)): List(:Item),
}

let right = List(Item: Nat);
let wrong = List(Nat);
