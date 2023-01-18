//! TODO: We will need to move this to the `should_succeed_with_warnings`
//! directory after we implement warnings for misordered labeled args.

type Nat {
    .o: Nat,
    .s(n: Nat): Nat,
}

type Bool {
    .true: Bool,
    .false: Bool,
}

let select = fun _(cond~c: Bool, true~t: Nat, ~false: Nat): Nat {
    match c {
        .true => t,
        .false => false,
    }
};

let _0 = Nat.o;
let _3 = Nat.s(Nat.s(Nat.s(_0)));

let right_order = select(cond: Bool.false, true: _0, false: _3);
let wrong_order = select(true: _0, cond: Bool.true, false: _3);
let wrong_order2 = select(true: _0, false: _3, cond: Bool.false);
let wrong_order3 = select(cond: Bool.true, false: _0, true: _0);
