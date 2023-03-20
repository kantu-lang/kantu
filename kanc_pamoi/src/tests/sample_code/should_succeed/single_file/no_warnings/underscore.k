type Unit {
    u: Unit,
}

type List(_: Type) {
    nil(T: Type): List(T),
    cons(T: Type, car: T, cdr: List(T)): List(T),
}

type TypeParamMultiple(_: Type, _: Unit) {}

type Foo {
    variant_param_multiple(_: Type, _: Unit): Foo,
}

let fun_param_multiple = fun fun_param_multiple_(_: Type, _: Foo): Unit {
    Unit.u
};

let variant_param_underscore_doesnt_perform_use_in_match_case =
    fun variant_param_underscore_doesnt_perform_use_in_match_case_(_: Type, foo: Foo): Unit {
        match foo {
            variant_param_multiple(_T, U') => U',
        }
    };

let match_case_param_multiple = match Foo.variant_param_multiple(Foo, Unit.u) {
    variant_param_multiple(_, _) => List.nil(Unit),
};

let fun_underscore = fun _(u: Unit): Unit {
    u
};
