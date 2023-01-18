type U {
    .u: U,
}

type Bool {
    .true: Bool,
    .false: Bool,
}

let foo = match U.u {
    .u => Bool.true,
    .u => Bool.false,
};
