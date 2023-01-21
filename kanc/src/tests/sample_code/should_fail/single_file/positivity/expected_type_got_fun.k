type Empty {}

type Unit {
    c: Unit,
}

type Bad {
    c(f: fun this_should_not_be_here(_: Unit): Type { forall(b: Bad) { Empty } }): Bad,
}
