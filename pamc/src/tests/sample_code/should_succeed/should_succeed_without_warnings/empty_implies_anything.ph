type Empty {}

type Empty2 {}

type Unit {
    .U: Unit,
}

type Bool {
    .False: Bool,
    .True: Bool,
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
        .True => match e1 {},
        .False => match e2 {},
    }
};

let empty_implies_empty2 = fun empty_implies_empty2(e: Empty): Empty2 {
    e
};
