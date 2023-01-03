type Empty {}

type Unit {
    .C: Unit,
}

type Bool {
    .True: Bool,
    .False: Bool,
}

let foo = fun _(e: Empty): Bool {
    match match e {} {
        .C => Bool.True,
    }
};
