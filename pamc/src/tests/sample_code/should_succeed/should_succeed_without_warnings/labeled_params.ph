type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

type Color {
    .C(~r: Nat, ~g: Nat, ~b: Nat): Color,
}

let redness = fun _(~c: Color): Nat {
    match c {
        // TODO: We will need to update this
        // when we add support for labeled
        // match case params
        .C(r, _, _) => r,
    }
};

// TODO: We will need to update this when we
// add support for labeled arguments.
let _1 = redness(Color.C(Nat.S(Nat.O), Nat.O, Nat.O));

let apply = fun _(c: Color, f: forall(~d: Color) { Nat }): Nat {
    // TODO: We will need to update this when we
    // add support for labeled arguments.
    f(c)
};

type List(~T: Type) {
    // TODO: We will need to update this when we
    // add support for labeled arguments.
    .Nil(U: Type): List(U),
    // TODO: We will need to update this when we
    // add support for labeled arguments.
    .Cons(U: Type, car: U, cdr: List(U)): List(U),
}

let foo = fun foo_(xylophone~x: Nat, yodeler~-y: Nat): Nat {
    Nat.O
};

let bar = fun bar_(~-a: Nat): Nat {
    Nat.O
};
