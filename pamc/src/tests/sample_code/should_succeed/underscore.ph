type Unit {
    .U: Unit,
}

type List(_: Type) {
    .Nil(T: Type): List(T),
    .Cons(T: Type, car: T, cdr: List(T)): List(T),
}

type TypeParamMultiple(_: Type, _: Unit) {}

type Foo {
    .VariantParamMultiple(_: Type, _: Unit): Foo,
}

let fun_param_multiple = fun fun_param_multiple_(_: Type, _: Foo): Unit {
    Unit.U
};

let variant_param_underscore_doesnt_perform_use_in_match_case =
    fun variant_param_underscore_doesnt_perform_use_in_match_case_(_: Type, foo: Foo): Unit {
        match foo {
            .VariantParamMultiple(_T, U') => U',
        }
    };

let match_case_param_multiple = match Foo.VariantParamMultiple(Foo, Unit.U) {
    .VariantParamMultiple(_, _) => List.Nil(Unit),
};

let fun_underscore = fun _(u: Unit): Unit {
    u
};
