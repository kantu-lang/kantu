type Empty {}

type Unit {
    .c: Unit,
}

type Eq(T: Type, a: T, b: T) {
    .refl(T: Type, t: T): Eq(T, t, t),
}

/*
data SET : Set where
set : (X : Set) → (X → SET) → SET
*/

type SET {
    .set(T: Type, H: forall(t: T) { SET }): SET,
}

/*
∅ : SET
∅ = set ⊥ ⊥-elim
*/

let null = SET.set(Empty, fun _(t: Empty): SET { match t {} });

/*
⟨∅⟩ : SET
⟨∅⟩ = set ⊤ (λ _ → ∅)
*/

let singleton = SET.set(Unit, fun _(_: Unit): SET { null });

/*
_∈_ : SET → SET → Set
a ∈ set X f = Σ X (λ x → a ≡ f x)

_∉_ : SET → SET → Set
a ∉ b = (a ∈ b) → ⊥
*/

type Exists(T: Type, P: forall(_: T) { Type }) {
    .c(T: Type, P: forall(_: T) { Type }, t: T, H: P(t)): Exists(T, P),
}

type In(a: SET, b: SET) {
    .c(
        a: SET,
        X: Type,
        f: forall(x: X) { SET },
        in: forall(x: X) { Eq(SET, a, f(x)) },
    ): In(a, SET.set(X, f)),
}

let NotIn = fun _(a: SET, b: SET): Type {
    forall(_: In(a, b)) { Empty }
};

/*
Δ : SET
Δ = set (Σ SET (λ s → s ∉ s)) proj₁
*/


let delta = SET.set(
    Exists(SET, fun _(s: SET): Type { NotIn(s, s) }),
    fun _(H: Exists(SET, fun _(s: SET): Type { NotIn(s, s) })): SET {
        match H {
            .c(_, _, t, _) => t,
        }
    },
);


/*
x∈Δ→x∉x : ∀ {X} → X ∈ Δ → X ∉ X
x∈Δ→x∉x ((Y , Y∉Y) , refl) = Y∉Y
*/

let delta_members_do_not_contain_self = fun _(x: SET, H: In(x, delta)): NotIn(x, x) {
    match H {
        .c(_a, _X, _f, _in) => fun _(H_x_in_x: In(x, x)): Empty {
            todo
        },
    }
};

/*
Δ∉Δ : Δ ∉ Δ
Δ∉Δ Δ∈Δ = x∈Δ→x∉x Δ∈Δ Δ∈Δ
*/

let delta_not_in_delta = fun _(H: In(delta, delta)): Empty {
    delta_members_do_not_contain_self(delta, H)(H)
};

/*
x∉x→x∈Δ : ∀ {X} →  X ∉ X → X ∈ Δ
x∉x→x∈Δ {X} X∉X = (X , X∉X) , refl
*/

let delta_in_delta = In.c(
    delta,
    Exists(SET, fun _(s: SET): Type { NotIn(s, s) }),
    fun _(H: Exists(SET, fun _(s: SET): Type { NotIn(s, s) })): SET {
        match H {
            .c(_, _, t, _) => t,
        }
    },
    fun _(x: Exists(SET, fun _(s: SET): Type { NotIn(s, s) })):
        Eq(
            SET,
            delta,
            fun _(H: Exists(SET, fun _(s: SET): Type { NotIn(s, s) })): SET {
                match H {
                    .c(_, _, t, _) => t,
                }
            }(x),
        )
    {
        todo
    },
);
