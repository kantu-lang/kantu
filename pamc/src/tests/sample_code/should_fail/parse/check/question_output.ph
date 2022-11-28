type Unit {
    .U: Unit,
}

let foo = check Unit.U: U {
    ?
};
