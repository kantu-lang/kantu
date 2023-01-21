type Bool {
    true: Bool,
    false: Bool,
}

let x = match Bool.true { .true => Bool, .false => Bool }.true;
