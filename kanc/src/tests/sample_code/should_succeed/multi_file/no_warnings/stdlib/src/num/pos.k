use super.*;

pub type Pos {
    one: Pos,
    extend(left: Pos, right: Bit): Pos,
}

pub let eq = fun eq(a: Pos, b: Pos): Bool {
    true
};
