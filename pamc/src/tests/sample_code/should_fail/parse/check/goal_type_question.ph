type Unit {
    .U: Unit,
}

let foo = fun _(_: Unit): Unit {
    check (goal: ?) {
        Unit.U
    }
};
