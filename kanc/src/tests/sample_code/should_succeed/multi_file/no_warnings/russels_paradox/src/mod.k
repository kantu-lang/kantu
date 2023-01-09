pub type Empty {}

pub type Unit {
    .c: Unit,
}

pub let(*) ascribe = fun _(T: Type, t: T): T {
    t
};

pub type Eq(T: Type, a: T, b: T) {
    .refl(T: Type, t: T): Eq(T, t, t),
}

/*
data SET : Set where
set : (X : Set) → (X → SET) → SET
*/

pub type SET {
    .set(T: Type, f: forall(t: T) { SET }): SET,
}

/*
∅ : SET
∅ = set ⊥ ⊥-elim
*/

pub let(*) null = SET.set(Empty, fun _(t: Empty): SET { match t {} });

/*
⟨∅⟩ : SET
⟨∅⟩ = set ⊤ (λ _ → ∅)
*/

pub let(*) singleton = SET.set(Unit, fun _(_: Unit): SET { null });

/*
_∈_ : SET → SET → Set
a ∈ set X f = Σ X (λ x → a ≡ f x)

_∉_ : SET → SET → Set
a ∉ b = (a ∈ b) → ⊥
*/

pub type Exists(T: Type, P: forall(_: T) { Type }) {
    .c(T: Type, P: forall(_: T) { Type }, t: T, H: P(t)): Exists(T, P),
}

pub type In(a: SET, b: SET) {
    .c(
        a: SET,
        X: Type,
        f: forall(x: X) { SET },
        H: Exists(X, fun _(x: X): Type { Eq(SET, a, f(x)) }),
    ): In(a, SET.set(X, f)),
}

pub let(*) NotIn = fun _(a: SET, b: SET): Type {
    forall(_: In(a, b)) { Empty }
};

/*
Δ : SET
Δ = set (Σ SET (λ s → s ∉ s)) proj₁
*/

pub let(*) delta_X = Exists(SET, fun _(s: SET): Type { NotIn(s, s) });
pub let(*) delta_f = fun _(H: delta_X): SET {
    match H {
        .c(_, _, t, _) => t,
    }
};
pub let(*) delta = SET.set(
    delta_X,
    delta_f,
);

/*
x∈Δ→x∉x : ∀ {X} → X ∈ Δ → X ∉ X
x∈Δ→x∉x ((Y , Y∉Y) , refl) = Y∉Y
*/

pub let(*) delta_members_do_not_contain_self = fun _(x: SET, H: In(x, delta)): NotIn(x, x) {
    match H {
        .c(_a, _X, _f, _Q) => fun _(H_x_in_x: In(x, x)): Empty {
            todo
        },
    }
};

/*
Δ∉Δ : Δ ∉ Δ
Δ∉Δ Δ∈Δ = x∈Δ→x∉x Δ∈Δ Δ∈Δ
*/

pub let(*) delta_not_in_delta = fun _(H: In(delta, delta)): Empty {
    delta_members_do_not_contain_self(delta, H)(H)
};

pub let(*) delta_wrapped = ascribe(delta_X, Exists.c(
    SET,
    fun _(s: SET): Type { NotIn(s, s) },
    delta,
    delta_not_in_delta,
));


/*
x∉x→x∈Δ : ∀ {X} →  X ∉ X → X ∈ Δ
x∉x→x∈Δ {X} X∉X = (X , X∉X) , refl
*/


pub let(*) delta_in_delta = In.c(
    delta,
    delta_X,
    delta_f,
    Exists.c(
        delta_X,
        fun _(x: delta_X): Type { Eq(SET, delta, delta_f(x)) },
        delta_wrapped,
        Eq.refl(SET, delta),
    ),
);


/*
pub let(*) delta_in_delta = In.c(
    delta,
    Exists(SET, fun _(s: SET): Type { NotIn(s, s) }),
    fun _(H: Exists(SET, fun _(s: SET): Type { NotIn(s, s) })): SET {
        match H {
            .c(_, _, t, _) => t,
        }
    },
    Exists.c(
        Exists(SET, fun _(s: SET): Type { NotIn(s, s) }),
        fun _(x: Exists(SET, fun _(s: SET): Type { NotIn(s, s) })): Type {
            Eq(
                SET,
                delta,
                fun _(H: Exists(SET, fun _(s: SET): Type { NotIn(s, s) })): SET {
                    match H {
                        .c(_, _, t, _) => t,
                    }
                }(x),
            )
        },
        todo,
        todo,
    ),
);
*/

// The function normalization is off.