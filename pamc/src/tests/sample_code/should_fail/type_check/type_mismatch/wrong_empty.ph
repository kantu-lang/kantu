type Empty1 {}
type Empty2 {}

let wrong_empty = fun _(e1: Empty1): Empty2 { e1 };
