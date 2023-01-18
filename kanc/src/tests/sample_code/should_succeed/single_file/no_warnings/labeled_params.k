type Nat {
    .o: Nat,
    .s(n: Nat): Nat,
}

type Color {
    .c(~r: Nat, ~g: Nat, ~b: Nat): Color,
}

let redness = fun _(~c: Color): Nat {
    match c {
        .c(:r, g: _, b: _) => r,
    }
};

let redness2 = fun _(~c: Color): Nat {
    match c {
        .c(:r, ...) => r,
    }
};

let redness3 = fun _(~c: Color): Nat {
    match c {
        .c(:r, g: _, ...) => r,
    }
};

let redness4 = fun _(~c: Color): Nat {
    match c {
        .c(...) => match c { .c(:r, ...) => r },
    }
};

// TODO: We'll need to move this file once we add
// misordered call arg warnings.
// Maybe match case param warnings, too, but I haven't
// checked yet (it's late and I want to get to bed rather
// than scrutinize every line of this file)
let _1 = redness(c: Color.c(r: Nat.s(Nat.o), b: Nat.o, g: Nat.o));

let apply = fun _(c: Color, f: forall(~d: Color) { Nat }): Nat {
    f(d: c)
};

let apply2 = fun _(c: Color, f: forall(c~_: Color) { Nat }): Nat {
    f(:c)
};

type List(~T: Type) {
    .nil(U: Type): List(T: U),
    .cons(U: Type, car: U, cdr: List(T: U)): List(T: U),
}

let foo = fun foo_(xylophone~x: Nat, yodeler~-y: Nat): Nat {
    Nat.o
};

let bar = fun bar_(~-a: Nat): Nat {
    Nat.o
};
