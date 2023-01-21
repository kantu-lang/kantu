type Nat {
    o: Nat,
    s(n: Nat): Nat,
}

type Color {
    c(~r: Nat, ~g: Nat, ~b: Nat): Color,
}

let pred = fun _(a: Nat): Nat {
    match a {
        o => Nat.o,
        s(a') => a',
    }
};

let plus = fun plus_(-a: Nat, b: Nat): Nat {
    match a {
        o => b,
        s(a') => Nat.s(plus_(a', b)),
    }
};
let mult = fun mult_(-a: Nat, b: Nat): Nat {
    match a {
        o => Nat.o,
        s(a') => plus(b, mult_(a', b)),
    }
};
let pow = fun pow_(~base: Nat, ~-power: Nat): Nat {
    match power {
        o => Nat.s(Nat.o),
        s(power') => mult(base, pow_(:base, power: power')),
    }
};

let _2 = Nat.s(Nat.s(Nat.o));
let _8 = mult(_2, mult(_2, _2));
let _256 = pow(base: _2, power: _8);
let _255 = pred(_256);

let white1 = Color.c(r: _255, g: _255, b: _255);

let white2 = fun _(r: Nat, g: Nat, b: Nat): Color {
    Color.c(:r, :g, :b)
}(_255, _255, _255);

let white3 = fun _(r: Nat, b: Nat): Color {
    Color.c(:r, g: _255, :b)
}(_255, _255);

let white4 = fun _(g: Nat): Color {
    Color.c(r: _255, :g, b: _255)
}(_255);

let redness = fun _(~c: Color): Nat {
    match c {
        c(:r, g: _, b: _) => r,
    }
};

let redness2 = fun _(~c: Color): Nat {
    match c {
        c(:r, ...) => r,
    }
};

let redness3 = fun _(~c: Color): Nat {
    match c {
        c(:r, g: _, ...) => r,
    }
};

let redness4 = fun _(~c: Color): Nat {
    match c {
        c(...) => match c { .c(:r, ...) => r },
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
    nil(U: Type): List(T: U),
    cons(U: Type, car: U, cdr: List(T: U)): List(T: U),
}

let foo = fun foo_(xylophone~x: Nat, yodeler~-y: Nat): Nat {
    Nat.o
};

let bar = fun bar_(~-a: Nat): Nat {
    Nat.o
};
