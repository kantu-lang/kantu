type U {
    .U: U,
}

type Bool {
    .True: Bool,
    .False: Bool,
}

let foo = match U.U {
    .U => Bool.True,
    .U => Bool.False,
};
