type Empty {}

type Empty2 {}

type Unit {
    u: Unit,
}

type Bool {
    false: Bool,
    true: Bool,
}

let identity = fun identity_(T: Type, t: T): T {
    t
};

let empty_implies_unit = fun empty_implies_unit_(e: Empty): Unit {
    match e {}
};

let empty_implies_unit2 = fun empty_implies_unit2_(e: Empty): Unit {
    identity(Unit, match e {})
};

let empty_implies_unit3 = fun empty_implies_unit3_(e1: Empty, e2: Empty2, b: Bool): Unit {
    match b {
        true => match e1 {},
        false => match e2 {},
    }
};

let empty_implies_empty2 = fun empty_implies_empty2(e: Empty): Empty2 {
    match e {}
};
