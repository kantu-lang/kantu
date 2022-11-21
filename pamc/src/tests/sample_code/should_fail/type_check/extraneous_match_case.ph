type Bool {
    .True: Bool,
    .False: Bool,
}

let foo = match Bool.True {
    .True => Bool.False,
    .Flaes => Bool.True,
};
