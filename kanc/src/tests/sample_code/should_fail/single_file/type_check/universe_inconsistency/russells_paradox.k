// Based on
// http://liamoc.net/posts/2015-09-10-girards-paradox.html
//
// See section "Russell's Paradox in Agda".

let ascribe = fun _(T: Type, t: T): T {
    t
};

type Equal(T: Type, l: T, r: T) {
    .refl(T: Type, t: T): Equal(T, t, t),
}

type Exists(T: Type, P: forall(_: T) { Type }) {
    .c(T: Type, P: forall(_: T) { Type }, witness: T, H: P(witness)): Exists(T, P),
}

type False {}

let Not = fun _(T: Type): Type {
    forall(_: T) { False }
};

type Set {
    .c(T: Type, mapper: forall(_: T) { Set }): Set,
}

type In(l: Set, R: Set) {
    .c(
        l: Set,
        RT: Type,
        r_mapper: forall(_: RT) { Set },
        H_in: Exists(RT, fun _(r_elem: RT): Type { Equal(Set, l, r_mapper(r_elem)) }),
    ): In(l, Set.c(RT, r_mapper)),
}

let NonSelfContaining = fun _(s: Set): Type {
    Not(In(s, s))
};
let NonSelfContainingSet = Exists(
    Set,
    NonSelfContaining,
);
let extract_nsc = fun _(s: NonSelfContainingSet): Set {
    match s {
        .c(_, _, witness, _) => witness,
    }
};

// Set of all non-self-containing sets.
let delta = Set.c(
    NonSelfContainingSet,
    extract_nsc,
);

/*let delta_members_are_nsc = fun _(s: Set, H: In(s, delta)): NonSelfContaining(s) {
    fun _(H_self_containing: In(s, s)): False {
        match H_self_containing {
            .c(
                _s,
                _s_RT,
                _s_r_mapper,
                H_exists,
            ) => match H_exists {
                .c(
                    _s_RT_2,
                    _,
                    s_r_elem,
                    H_s_r_elem_mapped_equals_set,
                ) => todo,
            },
        }
    }
};*/

let delta_members_are_nsc = fun _(s: Set, H_s_in_delta: In(s, delta)): NonSelfContaining(s) {
    fun _(H_self_containing: In(s, s)): False {
        match H_s_in_delta {
            .c(
                _s,
                _NonSelfContainingSet,
                _extract_nsc,
                H_exists_r_such_that_extracting_r_yields_s,
            ) => match H_exists_r_such_that_extracting_r_yields_s {
                .c(
                    _NonSelfContainingSet_2,
                    _ExtractingArgYieldsSPredicate,
                    r,
                    H_extracting_r_yields_s,
                ) => check (
                    r: NonSelfContainingSet,
                    H_extracting_r_yields_s: Equal(Set, s, extract_nsc(r)),
                ) {
                    // `r`'s "main type" is `_NonSelfContainingSet_2`, which
                    // is not an ADT itself.
                    // However, it is equal to `NonSelfContainingSet`, which
                    // _is_ an ADT.
                    // Thus, we must use type ascription before performing the
                    // match.
                    match ascribe(NonSelfContainingSet, r) {
                        .c(
                            _Set,
                            _NonSelfContaining,
                            r',
                            r'_not_in_r',
                        ) => check (
                            r': Set,
                            r'_not_in_r': NonSelfContaining(r'),
                        ) {
                            // Similarly, `H_extracting_r_yields_s`'s "main type" is
                            // `_ExtractingArgYieldsSPredicate(r)`, which
                            // is not an ADT itself.
                            // However, it is equal to `Equal(Set, s, r')`,
                            // which _is_ an ADT.
                            // So once again, we must use type ascription before
                            // performing the match.
                            match ascribe(
                                Equal(Set, s, r'),
                                H_extracting_r_yields_s,
                            ) {
                                // Once again, `r'_not_in_r'`'s "main type" is
                                // `_NonSelfContaining(r')`.
                                // So once again, we ascribe before matching.
                                .refl(_, _) => ascribe(NonSelfContaining(r'), r'_not_in_r')(
                                    H_self_containing
                                ),
                            }
                        },
                    }
                },
            },
        }
    }
};

let delta_not_in_delta = fun _(
    H_delta_in_delta: In(delta, delta),
): False {
    delta_members_are_nsc(delta, H_delta_in_delta)(H_delta_in_delta)
};

let wrapped_delta = Exists.c(
    Set,
    NonSelfContaining,
    delta,
    delta_not_in_delta,
);

type Unit { .trivial: Unit }
let _check = check (wrapped_delta: NonSelfContainingSet) { Unit.trivial };

let delta_in_delta = In.c(
    delta,
    NonSelfContainingSet,
    extract_nsc,
    Exists.c(
        NonSelfContainingSet,
        fun _(r_elem: NonSelfContainingSet): Type {
            Equal(Set, delta, extract_nsc(r_elem))
        },
        wrapped_delta,
        Equal.refl(Set, delta),
    ),
);

// let false = delta_not_in_delta(delta_in_delta);
