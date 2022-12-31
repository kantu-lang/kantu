type Bool {
    .True: Bool,
    .False: Bool,
}

let x = match Bool.True { .True => Bool, .False => Bool }.True;
