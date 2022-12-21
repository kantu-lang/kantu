type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

type Color {
    .C(~r: Nat, ~g: Nat, ~b: Nat): Color,
}

let redness = fun _(~c: Color): Nat {
    match c {
        .C(:r, g: _, b: _) => r,
    }
};

let redness2 = fun _(~c: Color): Nat {
    match c {
        .C(:r, ...) => r,
    }
};

let redness3 = fun _(~c: Color): Nat {
    match c {
        .C(:r, g: _, ...) => r,
    }
};

let redness4 = fun _(~c: Color): Nat {
    match c {
        .C(...) => match c { .C(:r, ...) => r },
    }
};

let _1 = redness(c: Color.C(r: Nat.S(Nat.O), b: Nat.O, g: Nat.O));

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
