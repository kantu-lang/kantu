type Bool {
    .True: Bool,
    .False: Bool,
}

type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

let foo = match Bool.True {
    .True => Nat.O,
    .False => Nat.S(Nat.O),
    .Maybe => Nat.S(Nat.S(Nat.O)),
};
