use super.bool.prelude.*;

pub type List(T: Type) {
    nil(T: Type): List(T),
    cons(T: Type, car: T, cdr: List(T)): List(T),
}

pub let(*) eq = fun eq(T: Type, -a: List(T), b: List(T), eqf: forall(ax: T, bx: T) { Bool }): Bool {
    match a {
        nil(_) => match b {
            nil(_) => true,
            cons(_, _, _) => false,
        },
        cons(_, a_car, a_cdr) => match b {
            nil(_) => false,
            cons(_, b_car, b_cdr) => match eqf(a_car, b_car) {
                true => eq(T, a_cdr, b_cdr, eqf),
                false => false,
            },
        },
    }
};