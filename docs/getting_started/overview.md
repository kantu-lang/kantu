# Pamlihu overview

## Identifiers

Identifier names can contain the following characters:

- Unicode letters
- Unicode numbers
- Unicode punctuation
- Unicode symbols

...with the exception of:

- The characters `;:,.@=-?~/()[]{}<>` cannot appear anywhere.
- The characters `0123456789` cannot appear as the first character, but may appear everywhere else.
- Whitespace cannot appear anywhere.

Additionally, the following strings are reserved words, and cannot be used as identifiers:

```
_ (the underscore)

type
let
Type
Type0
Type1
Type2
Type3
fun
match
forall
check
goal
impossible

struct
var
trait

pub
prot
priv
mod
pack
use
namespace

extern
unsafe
async

notation
exists

∀ (Unicode universal quantifier symbol)
∃ (Unicode existential quantifier symbol)
```

## `type` statements

Use the `type` keyword to declare types. Syntax:

```pamlihu
type TypeName(
    TypeParam0: TypeParamType0,
    TypeParam1: TypeParamType1,
    // ...
) {
    .Variant0(
        VariantParam0: VariantParamType0,
        VariantParam1: VariantParamType1,
        // ...
    ): TypeName(
        Variant0Output_TypeArg0,
        Variant0Output_TypeArg1,
        Variant0Output_TypeArg2,
        // ...
    ),

    .Variant1(
        VariantParam0: VariantParamType0,
        VariantParam1: VariantParamType1,
        // ...
    ): TypeName(
        Variant1Output_TypeArg0,
        Variant1Output_TypeArg1,
        Variant1Output_TypeArg2,
        // ...
    ),

    // ...
}
```

Examples:

```pamlihu
type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

type Bool {
    .True: Bool,
    .False: Bool,
}

type Rgb {
    // Use `~` to create labeled parameters (more on this later):
    .C(~r: Nat, ~g: Nat, ~b: Nat): Rgb,
}

type False {}

type List(T: Type) {
    .Nil(T: Type): List(T),
    .Cons(T: Type, car: T, cdr: List(T)): List(T),
}

// Dependent types are supported 🎉
type Equal(T: Type, x: T, y: T) {
    .Refl(T: Type, x: T): Equal(T, x, x),
}

type Or(L: Type, R: Type) {
    .Inl(L: Type, R: Type, l: L): Or(L, R),
    .Inr(L: Type, R: Type, r: R): Or(L, R),
}

let In = fun In(T: Type, item: T, list: List(T)): Type {
    match list {
        List.Nil(_T) => False,
        List.Cons(_T, car, cdr) => Or(Equal(T, item, car), In(T, item, cdr)),
    }
};

type LessThanOrEqualTo(L: Nat, R: Nat) {
    .Equal(n: Nat): LessThanOrEqualTo(n, n),
    .Step(a: Nat, b: Nat, H: LessThanOrEqualTo(a, b)): LessThanOrEqualTo(a, Nat.S(b)),
}

type ListOfEvenNats {
    .C(l: List(Nat), H_all_even: forall(n: Nat, H_in: In(Nat, n, l)) { Even(n) }): ListOfEvenNats,
}
```

Note that empty parameter lists are always omitted (so there is never `()`, outside of invocations).

### Type definition restrictions

Type declarations must be [_strictly positive_](https://cs.stackexchange.com/questions/55646/strict-positivity).
If you don't know what that means, don't worry--99% of types you
write will probably satisfy this requirement.
Odds are, the only time you will declare a type that violates
the strict positivity requirement is if you deliberately try to.

However, if you're still curious about (and you don't already know) the
definition of positivity, please read the article linked above.

#### Motivation of the strict positivity requirement

Without the requirement, we could write "broken" types that
allow us to prove false. For example:

```pamlihu
type False {}

type Broken {
    .B(f: forall(b: Broken) { False }): Broken,
}

let f = fun _(b: Broken): False {
    match b {
        .B(g) => g(b),
    }
};
let broken = Broken.B(f);
let false = f(broken);
// We just proved False! 😨
```

To prevent these "broken" types, we require types to be
strictly positive.
For example, in the above example, the `Broken.B` type variant
declaration would be rejected by the compiler, since `Broken`
appears in a negative position (i.e., `b: Broken`).

## `let` statements

```pamlihu
let N = Nat;
let O = Nat.O;
let S = Nat.S;
let _3 = S(S(S(O)));
```

Note that `let` aliases can't be used in `.` expressions.
For example, the following code will not compile:

```pamlihu
let N = Nat;
// Error: Invalid Dot expression LHS
let S = N.S;
```

## `match` expressions

The syntax is

```pamlihu
match matchee {
    .Variant0(param0_0, param0_1, param0_2, /* ... */) => case0_output,
    .Variant1(param1_0, param1_1, param1_2, /* ... */) => case1_output,
    // ...
}
```

Example:

```pamlihu
type Bool {
    .False: Bool,
    .True: Bool,
}

let false = match Bool.True {
    .False => Bool.True,
    .True => Bool.False,
};
```

Wildcards are not supported. Impossible cases must have the `impossible` keyword
written in place of the output.
For example:

```pamlihu
type TypeEq(A: Type, B: Type) {
    .Refl(C: Type): TypeEq,
}

type UnitX {
    .C: UnitX,
}

type UnitY {
    .C: UnitY,
}

type False {}

let f = fun _(H: TypeEq(UnitX, UnitY)): False {
    match H {
        .Refl(_) =>
        // This case is impossible, so rather than
        // write an output expression, we must write
        // `impossible`.
            impossible,
    }
};
```

## `fun` expressions (functions)

The syntax for a function expression is

```pamlihu
fun name(arg0: Type0, arg1: Type1, /* ... */): ReturnType {
    return_value
}
```

`fun`s must have at least one parameter.

Example:

```pamlihu
type Bool {
    .False: Bool,
    .True: Bool,
}

let not = fun not(b: Bool): Bool {
    match b {
        .False => Bool.True,
        .True => Bool.False,
    }
};
// We can now call the Function through the
// `let` binding. For example:
let true = not(Bool.False);
```

You can make functions anonymous by writing `_`
instead of a name.

```pamlihu
let not = fun _(b: Bool): Bool {
    match b {
        .False => Bool.True,
        .True => Bool.False,
    }
};
// We can still call the Function through the
// `let` binding--the function's name (or lack thereof)
// has no influence on the name of the binding.
let true = not(Bool.False);
```

It is strongly encouraged to make non-recursive functions anonymous.
The main purpose of allowing `fun` expressions to be named is to allow recursion.

### Recursive functions

Recursive functions must have a name (i.e., they cannot be anonymous), so
that they may be recursively called within their body.

Additionally, to prevent infinite recursion, they must also have a _decreasing parameter_.

A decreasing parameter is a parameter whose value must "decrease" with each recursive call.

The decreasing parameter must have a `-` before its name.
However, the `-` is **not** included as part of its name.

For example:

```pamlihu
let esoteric_identity_implementation = fun f(-n: Nat): Nat {
    match n {
        .O => Nat.O,
        .S(n') => Nat.S(f(n')),
    }
};
```

#### A more formal definition of "decreasing"

When you recursively call a function, you must pass in a _syntactic substructure_
of the decreasing parameter to the recursive call, in the same position.
A syntactic substructure to a value _n_ is any parameter that is either (1)
introduced (i.e., declared) by a `match n` expression, or (2) a syntactic substructure
of a parameter introduced by a `match n` expression.

For example, in

```pamlihu
let foo = fun _(n: Nat, m: Nat) {
    match n {
        .O => Nat.O,
        .S(n') =>
            match n' {
                .O => Nat.O,
                .S(n'') => Nat.O,
            }
    }
}
```

...the syntactic substructures of `n` are `n'` and `n''`.
By rule (1), `n'` is a syntactic substructure of `n` because
it is defined by an arm (specically, the `.S(n')` arm) of the `match n` expression.
Similarly, `n''` is a syntactic substructure of `n'` because it is
defined by an arm of the `match n'` expression.
Since `n'` is a syntactic substructure of `n`, and `n''` is a syntactic substructure
of `n'`, by rule (2), we conclude that `n''` is a substructure of `n`.

An error will be emitted if you either

1. Pass a non syntactic substructure to a decreasing parameter.
2. Recursively call a function that does not have a decreasing parameter defined.

All this may seem intimidating to non-functional programmers when discussed in the
abstract, so here are some concrete examples:

**Permitted:**

```pamlihu
let always_returns_zero = fun zero_(-n: Nat): Nat {
    match n {
        .O => Nat.O,
        .S(n') => zero_(n'),
    }
};
```

**Forbidden:**

```pamlihu
let infinite_recursion = fun f(-n: Nat): Nat {
    // The compiler will not permit this because
    // the first parameter of `f` is decreasing
    // (because of the `-` in `-n: Nat`), but
    // `n` is not a syntactic substructure of itself.
    f(n)
};
```

**Forbidden:**

```pamlihu
let no_decreasing_param = fun f(n: Nat): Nat {
    match n {
        .O => Nat.O,
        // Cannot recursively call `f` because
        // it does not have a decreasing parameter
        // defined (i.e., none of the parameters are
        // marked with `-`).
        .S(n') => f(n'),
    }
};
```

### Labeled parameters

You can also choose to make a function's parameters _labeled_.
Syntax:

```pamlihu
fun name(label0~param0: Type0, label1~param1: Type1, /* ... */): ReturnType {
    return_value
}
```

Example:

```pamlihu
let total_fruit = fun _(apples~a: Nat, bananas~ban: Nat, cherries~cher): Nat {
    plus(plus(a, ban), cher)
};
let x = total_fruit(apples: Nat.O, bananas: Nat.S(Nat.O), cherries: Nat.O);
```

If a label is the same as the parameter name, you can omit the label. For example,

```pamlihu
fun _(apples~apples: Nat): Nat {
    apples
}
```

and

```pamlihu
fun _(~apples: Nat): Nat {
    apples
}
```

are semantically the same.

A function must either have all unlabeled parameters or all labeled parameters--a mix is not allowed.
A function with unlabeled parameters is called an _unlabeled function_.
A function with labeled parameters is called a _labeled function_.

Call arguments should be labeled if and only if the function's parameters are labeled.
Both labeling arguments to an unlabeled function
or not labeling arguments to a labeled function is an error.

If you call a labeled function with the correct labels but in the wrong order, a warning will be
emitted, but it will not be an error.

#### Order of `~` and `-`

Rule: The `-` _always_ directly precedes the parameter name.

Quiz: Which two out of the following (i.e., A, B, C, D) are correct?

```pamlihu
let A = fun f(x~-x: Nat): Nat { x };
let B = fun f(~-x: Nat): Nat { x };
let C = fun f(-x~x: Nat): Nat { x };
let D = fun f(-~x: Nat): Nat { x };
```

Answer: A and B.

#### Are `fun` parameters the only parameters that can be labeled?

A: No. `forall`, type constructor, and type variant constructor parameters
can all be labeled.

#### Labels _and_ order are a part of the type!

Example:

```pamlihu
let f = fun _(~a: Nat): Nat { a }
let F = forall(~a: Nat) { Nat };
let expect_F = fun _(_: F): Unit { Unit.C };

// Okay: Labels of `f` match the labels of the required type (`F`).
let okay = expect_F(f);

let f' = fun _(a~b: Nat): Nat { b };
// Okay: Even though the parameter name is different (i.e., `f'`'s is `b` but `F`'s is `a`),
// the label still matches (both are `a`).
let also_okay = expect_F(f');

let unlabeled = fun _(a: Nat): Nat { a };
// Error: `expect_F` expects a labeled function,
// but `unlabeled` is an unlabled function
let wrong = expect_F(unlabeled);

let different_label = fun _(~b: Nat): Nat { b };
// Error: `expect_F` expects a function
// whose first parameter has a label `a`,
// but `different_label` is a function whose
// first parameter has a label `b`.
let also_wrong = expect_F(different_label);

```

If a function _f_ has the same labels as a forall _F_, but
the labels are in a different order, _f_ will **not** be considered
a member of type _F_.

Example:

```pamlihu
let f = fun _(~Texas: Type, ~Utah: Type, ~texas: Texas, ~utah: Utah): Unit { Unit.C };
let F = forall(~Texas: Type, ~Utah: Type, ~texas: Texas, ~utah: Utah) { Unit };
let expect_F = fun _(_: F): Unit { Unit.C };

// Okay: Both labels and order match.
let okay = expect_F(f);

let f' = fun(Texas~T: Type, Utah~U: Type, texas~t: T, utah~u: U): Unit { Unit.C };
// Okay: The parameter names are different, but once again,
// both the labels and order match.
let also_okay = expect_F(f);

let wrong_order = fun _(~Texas: Type, ~texas: Texas, ~Utah: Type, ~utah: Utah): Unit { Unit.C };
// Error: The labels are in the wrong order.
let wrong = expect_F(wrong_order);
```

In short, the labels _and_ the order must be the same.

## `forall` Expressions

Q: How do we express the type of a function?

A: We use `forall` expressions.

The syntax is

```pamlihu
forall (param0: Type0, param1: Type1, param2: Type2, /* ... */) { ReturnType }
```

Example:

```pamlihu
type Option(T: Type) {
    .None(T: Type): Option(T),
    .Some(T: Type, t: T): Option(T),
}

let map = fun _(T: Type, U: Type, o: Option(T), f: forall(t: T) { U }): Option(U) {
    match o {
        .None(_) => Option.None(U),
        .Some(_, t) => Option.Some(f(t)),
    }
};
```

`forall`s must have at least one parameter.

## Calling functions

Syntax:

```pamlihu
callee(arg0, arg1, arg2, /* ... */)
```

Alternatively, if the function has labeled parameters:

```pamlihu
callee(label0: arg0, label1: arg1, label2: arg2, /* ... */)
```

You can call type constructors, type variant constructors, and `fun`s. Example:

```pamlihu
type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

type Option(T: Type) {
    .None(T: Type): Option(T),
    .Some(T: Type, t: T): Option(T),
}

// Calling type constructor `Option` with arguments `(Nat)`:
let OptNat = Option(Nat);

// Calling type variant constructor `Nat.S` with arguments `(Nat.O)`:
let _1 = Nat.S(Nat.O);

let _2 = Nat.S(Nat.S(Nat.O));

/// Calling the `plus` function with arguments `(_1, _2)`:
let _3 = fun plus(-a: Nat, b: Nat): Nat {
    match a {
        .O => b,
        .S(a') => Nat.S(plus(a', b)),
    }
}(_1, _2);

let labeled_call_example = fun plus(~-a: Nat, bar~b: Nat): Nat {
    match a {
        .O => b,
        .S(a') => Nat.S(plus(a', b)),
    }
}(a: _1, bar: _2);
```

## `check` Expressions

`check` expressions are used to ask the compiler to check
certain equalities at compile time. They have zero runtime impact.

Syntax:

```pamlihu
check (
    // Type assertions:
    expr0: Type0,
    expr1: Type1,
    /* ... */,
    // Normal form assertions:
    expr'0 = expr''0,
    expr'1 = expr''1,
    /* ... */
) {
    output_expression
}
```

As you can see, you can pass in zero or more _type assertions_ and zero or more
_normal form assertions_.
Assertions may be in any order--type assertions are not required to come before
normal form assertions.

There must be at least one total assertions (i.e., `check () { output_expression }` is illegal).

Example:

```pamlihu
type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

type EqNat(x: Nat, y: Nat) {
    .Refl(z: Nat): EqNat(z, z),
}

let eq_comm = fun _(a: Nat, b: Nat, H: EqNat(a, b)): EqNat(b, a) {
    match H {
        .Refl(c) =>
            check (
                EqNat(b, a) = EqNat(c, c),
                EqNat.Refl(c): EqNat(c, c),
            ) {
                EqNat.Refl(c)
            },
    }
}
```

Semantically, a `check` expression immediately evaluates to its output (which is
why they have no runtime impact).

However, what is useful is that the compiler will produce warnings if any of the
assertions are incorrect.
Furthermore, a good compiler will generate corrections for the incorrect types/values.
This way, we can use `check` expressions to debug confusing type errors.

#### Using `goal` in a normal form assertion's LHS

For normal form assertions, you can write `goal` in place of
the left-hand side:

```pamlihu
type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

type EqNat(x: Nat, y: Nat) {
    .Refl(z: Nat): EqNat(z, z),
}

let eq_comm = fun _(a: Nat, b: Nat, H: EqNat(a, b)): EqNat(b, a) {
    match H {
        .Refl(c) =>
            check (
                // Observe that the LHS uses the `goal` keyword
                // instead of an expression
                goal = EqNat(c, c),

                EqNat.Refl(c): EqNat(c, c),
            ) {
                EqNat.Refl(c)
            },
    }
}
```

`goal` is the type that the current expression needs to take.

#### Using `?` in an assertion's RHS

For both type assertions and normal form assertions,
you can write `?` in place of the right-hand side:

```pamlihu
type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

type EqNat(x: Nat, y: Nat) {
    .Refl(z: Nat): EqNat(z, z),
}

let eq_comm = fun _(a: Nat, b: Nat, H: EqNat(a, b)): EqNat(b, a) {
    match H {
        .Refl(c) =>
            check (
                // Observe that the RHS uses the `?` operator
                // instead of an expression
                goal = ?,
                EqNat.Refl(c): ?,
            ) {
                EqNat.Refl(c)
            },
    }
}
```

Using a `?` will automatically fail the assertion
(thus triggering a warning--and on a good compiler/IDE,
an accompanying suggested correction).
This is useful when you want the compiler/IDE to provide
a solution for you.

## Comments

Single line:

```pamlihu
// This is a single line comment
let foo = bar(
    // Comments can pretty much go anywhere
    baz,
    quz,
);
```

Multiline:

```pamlihu
/* This is a
multiline
comment */

let foo = fun bar(
    n:
    /* /* Nested comments */ are supported! */
        Nat,
): Nat { n };
```
