type Boolish {
    true: Boolish,
    false: Boolish,
    maybe: Boolish,
}

let foo = match Boolish.true {
    true => Boolish.false,
};
