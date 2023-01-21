type Empty {}

type Unit {
    c: Unit,
}

type Bool {
    true: Bool,
    false: Bool,
}

let foo = fun _(e: Empty): Bool {
    match match e {} {
        c => Bool.true,
    }
};
