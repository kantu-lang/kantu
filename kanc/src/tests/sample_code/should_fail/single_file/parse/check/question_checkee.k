type Unit {
    .u: Unit,
}

let foo = check (?: Unit) {
    Unit.u
};
