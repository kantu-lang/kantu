type Empty {}

type Unit {
    .C: Unit,
}

type Bad {
    .C(f: fun this_should_not_be_here(_: Unit): Type { forall(b: Bad) { Empty } }): Bad,
}
