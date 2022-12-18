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

let _1 = redness(c: Color.C(Nat.S(Nat.O), Nat.O, Nat.O));

let apply = fun _(c: Color, f: forall(~d: Color) { Nat }): Nat {
    f(d: c)
};

let apply2 = fun _(c: Color, f: forall(c~_: Color) { Nat }): Nat {
    f(:c)
};

type List(~T: Type) {
    .Nil(U: Type): List(T: U),
    .Cons(U: Type, car: U, cdr: List(T: U)): List(T: U),
}

let foo = fun foo_(xylophone~x: Nat, yodeler~-y: Nat): Nat {
    Nat.O
};

let bar = fun bar_(~-a: Nat): Nat {
    Nat.O
};
