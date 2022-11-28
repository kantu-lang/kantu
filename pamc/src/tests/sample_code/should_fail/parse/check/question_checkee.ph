type Unit {
    .U: Unit,
}

let foo = check ?: Type {
    Unit.U
};
