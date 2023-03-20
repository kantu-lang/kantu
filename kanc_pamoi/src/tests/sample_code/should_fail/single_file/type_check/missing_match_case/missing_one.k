type Bool {
    true: Bool,
    false: Bool,
}

let foo = match Bool.true {
    true => Bool.false,
};
