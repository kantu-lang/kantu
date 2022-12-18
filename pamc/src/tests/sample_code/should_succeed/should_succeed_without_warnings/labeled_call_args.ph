type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

type Color {
    .C(~r: Nat, ~g: Nat, ~b: Nat): Color,
}

let pred = fun _(a: Nat): Nat {
    match a {
        .O => Nat.O,
        .S(a') => a',
    }
};

let plus = fun plus_(-a: Nat, b: Nat): Nat {
    match a {
        .O => b,
        .S(a') => Nat.S(plus_(a', b)),
    }
};
let mult = fun mult_(-a: Nat, b: Nat): Nat {
    match a {
        .O => Nat.O,
        .S(a') => plus(b, mult_(a', b)),
    }
};
let pow = fun pow_(~base: Nat, ~-power: Nat): Nat {
    match power {
        .O => Nat.S(Nat.O),
        .S(power') => mult(base, pow_(:base, power: power')),
    }
};

let _2 = Nat.S(Nat.S(Nat.O));
let _8 = mult(_2, mult(_2, _2));
let _256 = pow(_2, _8);
let _255 = pred(_256);

let white1 = Color.C(r: _255, g: _255, b: _255);

let white2 = fun _(r: Nat, g: Nat, b: Nat): Color {
    Color.C(:r, :g, :b)
}(_255, _255, _255);

let white3 = fun _(r: Nat, b: Nat): Color {
    Color.C(:r, g: _255, :b)
}(_255, _255);

let white4 = fun _(g: Nat): Color {
    Color.C(r: _255, :g, b: _255)
}(_255);

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
