type Unit {
    .U: Unit,
}

let foo = check (?: Unit) {
    Unit.U
};
