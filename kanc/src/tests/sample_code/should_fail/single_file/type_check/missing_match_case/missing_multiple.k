type Boolish {
    .True: Boolish,
    .False: Boolish,
    .Maybe: Boolish,
}

let foo = match Boolish.True {
    .True => Boolish.False,
};
