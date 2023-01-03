//! TODO: We will need to move this to the `should_succeed_with_warnings`
//! directory after we implement warnings for misordered labeled args.

type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

type Bool {
    .True: Bool,
    .False: Bool,
}

let select = fun _(cond~c: Bool, true~t: Nat, ~false: Nat): Nat {
    match c {
        .True => t,
        .False => false,
    }
};

let _0 = Nat.O;
let _3 = Nat.S(Nat.S(Nat.S(_0)));

let right_order = select(cond: Bool.False, true: _0, false: _3);
let wrong_order = select(true: _0, cond: Bool.True, false: _3);
let wrong_order2 = select(true: _0, false: _3, cond: Bool.False);
let wrong_order3 = select(cond: Bool.True, false: _0, true: _0);
