# Kantu language guide

## Disclaimer

This document is still under construction.
It may not be 100% up-to-date (though it should be at least 90% accurate).
It also needs to be simplified, since it's really verbose (and probably a bit intimidating to newcomers) right now.

## Identifiers

Identifier names can contain the following characters:

- Unicode letters
- Unicode numbers
- Unicode punctuation
- Unicode symbols

...with the exception of:

- The characters `;:,.@=-?~/*()[]{}<>` cannot appear anywhere.
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
todo

struct
var
trait

pub
prot
priv
mod
super
super2
super3
super4
super5
super6
super7
super8
pack
use
as
namespace

extern
unsafe
async

notation
exists

∀ (Unicode universal quantifier symbol)
∃ (Unicode existential quantifier symbol)
```

### Name shadowing

Name shadowing (declaring two variables with the same name) is strictly forbidden.

However, keep in mind that the binding declared by a `let` statement does not go into scope until _after_ the statement is completely finished, making it possible to write things like this:

```kantu
let foo = fun foo(n: Nat): Nat {
    // The `let foo` statement is not complete,
    // so that `foo` is not in scope.
    // Only the `foo` defined by the `fun foo<...>`
    // expression is in scope.
    // Thus, there are no conflicting names,
    // and this code is legal.

    check (foo: forall(x: Nat) { Nat }) {
        Nat.S(n)
    }
}(Nat.O);

// Now the `let foo` statement in complete,
// so that `foo` is now in scope.
// The `foo` defined by the `fun foo` expression
// is only in scope within the body of the `fun` expression,
// so it is not in scope here.
// Thus, there are no conflicting names,
// and this code is legal.
let y = check (foo = Nat.S(Nat.O)) {
    foo
};
```

This "not in scope until declaration fully ends" rule
also applies to type parameters, variant parameters function parameters, and forall parameters.

So the below code is legal (although strongly discouraged, since it's not very readable):

```kantu
type Foo(a: forall(a: Nat) { Nat }) {}

type Bar {
    .BarVariant(a: forall(a: Nat) { Nat }): Bar,
}

let x1 = fun _(f: fun _(f: Nat): Type { Nat }(Nat.O)): Nat {
    f
};

let x2 = forall(f: forall(f: Nat) { Nat }) { Unit };
```

Also, keep in mind that type parameters are not in scope within the type variant declarations, so this is perfectly legal, clean code:

```kantu
type List(T: Type /* The `T` defined here...*/) {
    // ...and is NOT in scope here.
    .Nil(
        // Thus, we are free to name another
        // parameter `T`, since this does NOT
        // cause any name conflict:
        T: Type,
    ): List(T),

    // Same thing here
    .Cons(T: Type, car: T, cdr: List(T)): List(T),
}
```

## `type` statements

Use the `type` keyword to declare types. Syntax:

```kantu
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

```kantu
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

type Empty {}

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
        List.Nil(_T) => Empty,
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

Kantu does not have the concept of a nullary function.
As a result, empty parameter lists `()` are syntactically invalid.

Since Kantu is pure and total, there is no need for nullary
functions--any time you want to write `fun _() { some_val }`,
you should simply write `some_val`.

### Type definition restrictions

Type declarations must pass a _positivity test_.
This test is based on the notion of [strict positivity](https://cs.stackexchange.com/questions/55646/strict-positivity).
However, Kantu imposes additional restrictions in order to simplify the positivity testing algorithm.

You can probably skip this section, since odds are, the only time you will declare a type that violates
the strict positivity requirement is if you deliberately try to.

However, if you're still curious about this (or you got referred to
this page by a StackOverflow answer because you were one of the unlucky
one in a million who stumbled upon this error "out in the wild"), read on.

#### Motivation of the strict positivity requirement

Without the requirement, we could write "broken" types that
allow us to prove false. For example:

```kantu
type False {}

type Broken {
    .C(f: forall(b: Broken) { False }): Broken,
}

let f = fun _(b: Broken): False {
    match b {
        .C(g) => g(b),
    }
};
let broken = Broken.C(f);
let false = f(broken);
// We just proved False! 😨
```

To prevent these "broken" types, we forbid recursive
references in any `forall` parameter type.
For example, the above example would be rejected by
the positivity checker because...

```kantu
type Broken {
    .C(
        f:
            // ...`Broken` appears in the parameter type
            // of the forall parameter `b`.
            forall(b: Broken) { False },
    ): Broken,
}
```

However, this restriction is not enough!
If we only had this restriction, we could circumvent it,
such as in the code below

```kantu
type False {}

type Not(T: Type) {
    .C(f: forall(_: T) { False }): Not(T),
}

type Broken {
    // Look! `Broken` does not appear in a
    // a forall parameter type!
    // So this code is safe, right?...
    .C(n: Not(Broken)): Broken,
}

let f = fun _(b: Broken): False {
    match b {
        .C(n) =>
            match n {
                .C(g) => g(b),
            },
    }
};
let not_broken = Not.C(f);
let broken = Broken.B(not_broken);
let false = f(broken);
// Once again, we proved False! 😨
```

As you can see, by defining additional types such as `Not`,
we can circumvent the direct forall restriction by getting
the recursive reference to _indirectly_ appear in a forall
parameter type, as we did above.

To prevent sneaky techniques like the one above, we need
to impose additional restrictions.

The final list of rules is listed
below.
This section is already getting long, so I won't explain
the rationale behind each rule in this article.

#### Positivity rules

1. A type declaration `type T(...) {...}` is considered
   to _pass the positivity test_ if for every variant `V_i`,
   the proposition `check_variant(V_i, T)` holds.
2. **Definition of `check_variant(V_i, T)`:** True if and only if
   for every param type `pt_j`, the proposition `check_expr_pos(pt_j, T)`
   holds.
3. **Definition of `check_expr_pos(x, T)`:**
   1. If `x` is a name, this is true.
   2. If `x` is a `fun`, this is false.
   3. If `x` is a `forall`, this is true if and only if
      `T` does not appear in any of x's param types, and
      `x` has an output `x_output` such that
      `check_expr_pos(x_output, T)` holds.
   4. If `x` is a `match`, this is true if and only if
      `T` does not appear in the matchee, and each match case
      has an output `out_k` such that `check_expr_pos(out_k, T)`
   5. If `x` is a `call` where `T` never appears, this is true.
   6. If `x` is a `call` where `T` _does_ appear, and the
      callee is a type constructor `T2`, then this is true if
      and only if for each argument `arg_i` where `T` appears,
      the proposition `check_type_param_pos(T2, i)` holds.
   7. If `x` is a `call` where `T` _does_ appear, but the
      callee is not a type constructor, this is false.
4. **Definition of `check_type_param_pos(T, i)`:**
   True if and only if for every variant `V`, the proposition
   `check_type_param_pos_in_variant(V, i, T)` holds.
5. **Definition of `check_type_param_pos_in_variant(V, i, T)`:**
   Let `arg_i` be the `i`th argument of the variant's return type.
   If none of `V`'s parameters appear in `arg_i`, this is true.
   Otherwise, if `arg_i` contains at least one of `V`'s parameter
   but is not a name, this is false.
   Otherwise, `arg_i` must be a name AND contain at least one parameter--in
   other words, `arg_i` _is_ a reference to some parameter, call it param `p_j`.
   This is true if and only if for each param after param `p_j`, the param type
   `pt_k` satisfies `check_expr_pos{i, T}(pt_k, p_j)` where `check_expr_pos{i, T}`
   is `check_expr_pos` except it automatically substitutes "True" for any
   use of `check_type_param_pos(T, i)` within its algorithm.

Once again, this may feel like a math textbook, so don't worry too much
about it--you'll probably never need it.

## `let` statements

```kantu
let N = Nat;
let O = Nat.O;
let S = Nat.S;
let _3 = S(S(S(O)));
```

Note that `let` aliases can't be used in `.` expressions.
For example, the following code will not compile:

```kantu
let N = Nat;
// Error: Invalid Dot expression LHS
let S = N.S;
```

## `match` expressions

The syntax is

```kantu
match matchee {
    .Variant0(param0_0, param0_1, param0_2, /* ... */) => case0_output,
    .Variant1(param1_0, param1_1, param1_2, /* ... */) => case1_output,
    // ...
}
```

Example:

```kantu
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

```kantu
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

```kantu
fun name(arg0: Type0, arg1: Type1, /* ... */): ReturnType {
    return_value
}
```

`fun`s must have at least one parameter.

Example:

```kantu
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

```kantu
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

```kantu
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

```kantu
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

```kantu
let always_returns_zero = fun zero_(-n: Nat): Nat {
    match n {
        .O => Nat.O,
        .S(n') => zero_(n'),
    }
};
```

**Forbidden:**

```kantu
let infinite_recursion = fun f(-n: Nat): Nat {
    // The compiler will not permit this because
    // the first parameter of `f` is decreasing
    // (because of the `-` in `-n: Nat`), but
    // `n` is not a syntactic substructure of itself.
    f(n)
};
```

**Forbidden:**

```kantu
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

```kantu
fun name(label0~param0: Type0, label1~param1: Type1, /* ... */): ReturnType {
    return_value
}
```

Example:

```kantu
let total_fruit = fun _(apples~a: Nat, bananas~ban: Nat, cherries~cher): Nat {
    plus(plus(a, ban), cher)
};
let x = total_fruit(apples: Nat.O, bananas: Nat.S(Nat.O), cherries: Nat.O);
```

If a label is the same as the parameter name, you can omit the label. For example,

```kantu
fun _(apples~apples: Nat): Nat {
    apples
}
```

and

```kantu
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

It is good practice to pass labeled arguments in the same order that
the labels appear in the parameters of the defining type.

For example:

```kantu
type Nat {
    .O: Nat,
    .S(_: Nat): Nat,
}

type Color {
    // Observe that the param label order is
    // "r,g,b".
    .C(~r: Nat, ~g: Nat, ~b: Nat): Color,
}

let O = Nat.O;

// In this call expression, the argument label
// order matches the param label order (i.e.,
// both are "r,g,b")
let good = Color.C(r: O, g: O, b: O);

// In this call expression, the argument label
// order does NOT match the param label order (i.e.,
// param order is "r,g,b" but the arg order is
// "b,g,r").
let still_legal_but_frowned_upon = Color.C(b: O, g: O, r: O);
```

Calling a labeled function with correctly labeled but misordered arguments (e.g., like `still_legal_but_frowned_upon` in the above example) will **always be legal**.

However, in a future version of Kantu, this may result in warnings.

#### Order of `~` and `-`

Rule: The `-` _always_ directly precedes the parameter name.

Quiz: Which two out of the following (i.e., A, B, C, D) are correct?

```kantu
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

```kantu
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

```kantu
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

#### Writing `match` cases on variants with labeled parameters

If you try writing the following code, you will get an error

```kantu
type Color {
    .C(~r: Nat, ~g: Nat, ~b: Nat): Color,
}

let redness_WRONG = fun _(c: Color): Nat {
    match c {
        // Error:
        // Variant has labeled parameters
        // but match case has unlabeled parameters.
        .C(red, _, _) => red,
    }
};
```

This is because `match` cases corresponding to variants with labeled parameters must have labeled parameters themselves.

To fix this code, one could write

```kantu
let redness = fun _(c: Color): Nat {
    match c {
        .C(r: red, g: _, b: _) => red,
    }
};
```

##### `...` syntax

Writing `g: _, b: _` may be a hassle
(especially if you have many parameters), so
you can alternatively write

```kantu
let redness2 = fun _(c: Color): Nat {
    match c {
        .C(r: red, ...) => red,
    }
};
```

The `...` must go at the end of the parameter list (e.g., `.C(..., r: red)` is illegal).

##### Implicit label syntax

Similar to how `~foo` can be used as shorthand for
`foo~foo`, we can also use `:foo` as shorthand for
`foo: foo`.

In the above example, if we used the name `r` instead of `red`, then we would have `r: r`, which
would could be shortened to `:r`:

```kantu
let redness3 = fun _(c: Color): Nat {
    match c {
        .C(:r, ...) => r,
    }
};
```

##### Matching param label order is strongly encouraged (but not required)

```kantu
type Nat {
    .O: Nat,
    .S(_: Nat): Nat,
}

type Color {
    // Observe that the param label order is
    // "r,g,b".
    .C(~r: Nat, ~g: Nat, ~b: Nat): Color,
}

let O = Nat.O;

let good = fun _(c: Color): Nat {
    match c {
        // In this match case, the match case param label
        // order matches the type variant param label order (i.e.,
        // both are "r,g,b")
        .C(:r, :g, :b): r,
    }
};

let still_legal_but_frowned_upon = fun _(c: Color): Nat {
    match c {
        // In this match case, the match case param label
        // order does NOT match the type variant param label order
        // (i.e., the variant's order "r,g,b", but the case's
        // order is "b,g,r").
        .C(:b, :g, :r): r,
    }
};
```

It will always be legal to declare correctly labeled but misordered
match case parameters (e.g., like in `still_legal_but_frowned_upon` in the
above example).

However, in a future version of Kantu, doing this may trigger some warnings.

## `forall` Expressions

Q: How do we express the type of a function?

A: We use `forall` expressions.

The syntax is

```kantu
forall (param0: Type0, param1: Type1, param2: Type2, /* ... */) { ReturnType }
```

Example:

```kantu
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

```kantu
callee(arg0, arg1, arg2, /* ... */)
```

Alternatively, if the function has labeled parameters:

```kantu
callee(label0: arg0, label1: arg1, label2: arg2, /* ... */)
```

You can call type constructors, type variant constructors, and `fun`s. Example:

```kantu
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

```kantu
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

```kantu
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

```kantu
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

```kantu
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

#### Syntactically well-formed assertions will not produce compiler errors

Since check assertions are intended to be like "interactive comments", the compiler doesn't really
care what you write in them, as long as they are syntactically correct.

> Obviously, if you write a syntactically *in*correct
> check assertion, then the parser will not know how to parse it.
> Thus, even though check assertions are designed to be as lenient as possible (with respect to emitting errors),
> we must at least require check assertions
> to be syntactically well-formed (i.e., correct).
> Fortunately, this is an incredibly low bar to meet.

As a result, the below code will generate warnings,
but no errors:

```kantu
type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

let foo = fun _(n: Nat): Nat {
    check (
        // Undefined names (`b` and `c`)
        b = c,
        // Ill typed RHS
        goal: Nat.S(Nat),
        g = fun infinite_loop(a: Nat): Nat {
            infinite_loop(a)
        },
    ) {
        Nat.O
    }
};
```

As you can see, there were numerous problems in the
above code, such as references to undefined names
(e.g., `b`, `c`), ill-typed terms `Nat.S(Nat)`, and illegal recursion.
All these problems would normally result in errors.
However, check assertions are meant to serve as
compiler-checked documentation--like comments, they don't have any
impact on the semantic meaning of the code.
Thus, the compiler demotes these would-be errors
into mere warnings, since they do not stop otherwise
correct code from being compiled.

The reason the compiler generates warnings at all is
because if your check assertions are ill-formed, it
is incorrect documentation, which almost always (1) indicates buggy code, and (2) misleads developers who read said documentation.
Consequently, one is encouraged to fix the warnings when they get the time.

## Comments

Single line:

```kantu
// This is a single line comment
let foo = bar(
    // Comments can pretty much go anywhere
    baz,
    quz,
);
```

Multiline:

```kantu
/* This is a
multiline
comment */

let foo = fun bar(
    n:
    /* /* Nested comments */ are supported! */
        Nat,
): Nat { n };
```

## Errors vs. warnings

When the compiler spots a problem, it has 2 modes of reporting it:

1. Errors - Your code can _never_ build while it has one or more errors.
2. Warnings - By default, the compiler will reject code with one or more warnings, but you can run the compiler in "lax mode" to force it to build in spite of warnings.

The motivation behind having this distinction between errors and warnings is there are often
bad coding practices (e.g., unused variables) that we want the compiler to alert us of, but that we want the compiler to also be able to ignore.

For example, imagine writing some new code which
results in previously used variables no longer being used.
You want to test this code _now_, and only go back and fix
the problems (e.g., remove the unused variables) later.
A compiler will warn you of the problems, but it shouldn't _force_ you to fix them if it doesn't have to (i.e., if it's still capable
of building an executable despite the problems).
This is what warnings are for.

On the other hand, we also have times when
code problems are so severe there is no sane way
to build an executable (e.g., we reference and undefined name). These are errors.

## Projects with multiple files

We'll begin this section by examining an example project.

### Example project:

#### File layout:

```text
pack.omlet
src/
    mod.ph
    foo.ph
    bar/
        mod.ph
        baz.ph
```

In Kantu, the smallest "shareable" unit of code is the _package_.
Kantu packages are roughly analogous to npm packages or Rust crates.

A package consists of a `pack.omlet` file and one or more
`.ph` files.
The `.ph` files are held in a `src` directory.

Packages are broken down into a tree of smaller components called _modules_.
In fact, the package _is itself_ a module.
We will subsequently refer to this module as "the package's root module",
or simply "the package".

Generally, each `.ph` file corresponds to a module.

The top-level `mod.ph` file (i.e., the one in the same directory as `pack.omlet`) corresponds to the root module.

#### `pack.omlet`:

```omlet
kantu_version = 1.0.0
```

`pack.omlet` can be used to list external dependencies and specify
compiler options.
In this example project, we do not have any external dependencies, and
we use all the default compiler options, so we simply write `{}` to
denote an "empty" config.

#### `src/mod.ph` (corresponds to the `pack` module):

```kantu
mod foo;
pub mod bar;

pub use foo.Nat;
use Nat.O;
use Nat.S;

pub let factorial = fun f(-a: Nat): Nat {
    match a {
        .O => S(O),
        .S(a') => bar.mult(a, f(a')),
    }
};
```

This is file corresponding to the `pack` module (i.e., the package).
If you want any items to be accessible to
consumers of this package, you need to
mark them as _publicly visible_
(in this example, we make the module `bar`,
the alias `Nat`
and the constant `factorial` public).

To change the visibility of an item, use the `pub` keyword.
More details on this will be covered in the **Module item visibility** section.

If an item does not have a `pub` prefix, its visibility defaults to
`pub(mod)`.
In this example, `foo`, `O`, `S`, and `mult`
all implictly have `pub(mod)` visibility.

To use an item that doesn't directly belong to this module, you must use fully qualified syntax (e.g., like `bar.mult`, in the above example).

However, you can create aliases using the `use` keyword, like the
above example does with `foo.Nat`, `Nat.O`, and `Nat.S`.
Creating an alias will make the aliased item available from the
current module's namespace.
As with all file items, the visibility defaults to `pub(mod)`,
but you can use `pub` or `pub(<insert_modifier_here>)` to
change it.

#### `src/foo.ph` (corresponds to the `pack.foo` module):

```kantu
pub type Nat {
    .O: Nat,
    .S(_: Nat): Nat,
}
```

#### `src/bar/mod.ph` (corresponds to the `pack.bar` module):

```kantu
use super.Nat;

pub let mult = fun f(-a: Nat, b: Nat): Nat {
    match a {
        .O => super.O,
        .S(a') => plus(b, f(a', b)),
    }
};
```

Note that since we created the aliases `Nat` and `O`
back in `mod.ph`, and `pack` is this module's supermodule,
we can now access those aliases here
by writing `super.Nat` and `super.O`.
However, if we find that that's still too long to write,
we can create an _alias to that alias_ by writing
`use super.Nat;`.
Note that we decline to alias `super.O`, so when we reference
it (i.e., in the output of the match expression's `.O` case),
we use fully qualified syntax.

#### `src/bar/baz.ph` (corresponds to the `pack.bar.baz` module):

```kantu
use super.Nat;
use super2.S;

pub(super) let plus = fun f(-a: Nat, b: Nat): Nat {
    match a {
        .O => b,
        .S(a') => S(f(a', b)),
    }
};
```

Note that we have to write `use super2.S;` instead of simply
`use super.S;`.
This is because the supermodule (i.e., `pack.bar`) does _not_ export
an item named `S`, but the supermodule's supermodule (i.e., `pack`)
does indeed export such an item.

#### End of the example

This concludes the example project.
The following sections will summarize old topics
and/or add additional details.

### Packages and modules

A package is a unit of distributable code, like a crate in Rust.
A package is a module, called the `pack` module.
Modules can contain zero or more submodules.

### Terminology clarification: submodules, descendant modules, ancestor modules, oh my!

Modules can be organized into a tree, with the `pack` module at the root.

- A _submodule_ of `module_x` is a (direct) child `module_x`.
- A _supermodule_ of `module_x` is the (direct) parent of `module_x`.
- A _descendant module_ (or simply _descendant_ for short) of `module_x` is, as the name suggests, a descendant of `module_x`. That is, it is either a child, or a child of a child, or a child of a child of a child, etc.
- A _ancestor module_ (or simply _ancestor_ for short) of `module_x` is, as the name suggests, an ancestor `module_x`. That is, it is either a parent, or a parent of a parent, or a parent of a parent of a parent, etc.

### How to create a package

1. Create a `pack.omlet` file.
2. In the same directory, create a `src/` directory.
3. In that `src` directory, create a `mod.ph` file.

The `mod.ph` file corresponds to the `pack` module.
The `src` directory is referred to as the package's _source directory_.
Not surprisingly, all submodules of `pack` will go in the package's source directory, submodules of those submodules will go in subdirectories of the
source directory, submodules of _those_ submodules will go in subdirectories of _those_ subdirectories, and so on--turtles all the way down.

### How to create a module

There are two ways to create a module `foo`:

1. Create a file `foo.ph`. Then add `mod foo;` to the supermodule file.
2. Create a file `foo/mod.ph`. That is, create a `foo/` directory
   and create a `mod.ph` file inside that directory.

   Then add `mod foo;` to the supermodule file.

   Note: You **must** choose this option if you want `foo` to have submodules.

Create the files in the directory containing the `mod.ph` file of
`foo`'s desired parent.

For example, if you to create `pack.foo` (i.e., you want `foo` to be the child of `pack`), then you should create its file(s) in the directory of the
`mod.ph` file corresponding to `pack`.
In other words, create the file(s) in the package's source directory (i.e., `src/`).
Then, you would add `mod foo;` to `src/mod.ph`.

As another example, if you want to create `pack.bar.baz.foo`
(i.e., you want `foo` to be the child of `pack.bar.baz`), then
create the file(s) in the `src/bar/baz/` directory.
Then, you would add `mod foo;` to `src/bar/baz/mod.ph`.

### Module items

Each module has zero or more _items_.
An item can be a

- Constant (declared using `let`)
- Type (declared using `type`)
- Submodule (declared using `mod`)
- Alias (declared using `use`)

The current module's items are automatically in the current module's
scope.
Other modules' items must be referenced using `.` syntax.

Example:

`src/mod.ph`

```kantu
pub mod nat;
pub mod factorial;
```

`src/nat.ph`

```kantu
type Nat {
    .O: Nat,
    .S(_: Nat): Nat,
}
```

`src/factorial/mod.ph`

```kantu
mod plus;

// `Nat` was declared by a module other than the current module,
// so we must use `.` syntax (specifically, `super.nat.Nat`).
let mult = fun mult(-a: super.nat.Nat, b: super.nat.Nat): super.nat.Nat {
    match a {
        .O => super.nat.Nat.S(super.nat.Nat.O),
        .S(a') =>
            // `plus` was declared by a module other than the current module,
            // so we must use `.` syntax (specifically, `plus.plus`).
            plus.plus(b, mult(a', b)),
    }
};

pub let factorial = fun factorial(-a: super.nat.Nat): super.nat.Nat {
    match a {
        .O => super.nat.Nat.S(super.nat.Nat.O),
        // `mult` is declared by the current module, so we don't
        // need `.` syntax.
        .S(a') => mult(a, factorial(a')),
    }
};
```

`src/factorial/plus.ph`:

```kantu
// `Nat` was declared by a module other than the current module,
// so we must use `.` syntax (specifically, `super2.nat.Nat`).
pub let plus = fun plus(-a: super2.nat.Nat, b: super2.nat.Nat): super2.nat.Nat {
    match a {
        .O => b,
        .S(a') => super2.nat.Nat.S(plus(a', b)),
    }
};
```

### Module item visibility

By default, module items can only be accessed within
their declaring module and all descendant modules.

However, you can prefix a item declaration statement with the `pub`
keyword to modify that item's visibility.

| Visibility level                                      | Description                                                                                                                                                                                                                                                                                                                      |
| ----------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `pub` or `pub(*)`                                     | Global visibility. Any module in the world can access this item.                                                                                                                                                                                                                                                                 |
| `pub(mod)`                                            | Module visibility. This is the default visibility level. Only the declaring module and its descendants can access this item. You should never explicitly write this, since this is the visibility an item defaults to if no `pub` is written at all.                                                                             |
| `pub(super)`                                          | Supermodule visibility. Only the declaring module's supermodule and the supermodule's descendants can access this.                                                                                                                                                                                                               |
| `pub(super2)`, `pub(super3)`, ... up to `pub(super8)` | `super2` refers to the supermodule's supermodule. `super3` refers to the supermodule's supermodule's supermodule. ...and so on. You can write any `superN` where `2 ≤ N ≤ 8` (assuming such a module exists, of course). Beyond 8, you have to use `pub(pack.some.arbitrary.module)` syntax, which is described in the next row. |
| `pub(pack.some.arbitrary.module)`                     | The specified module _must_ be an ancestor of the declaring module (otherwise the item wouldn't be visible to itself, which obviously makes no sense).                                                                                                                                                                           |

### Declaring aliases with `use`

If you don't want to type a bunch of `.`s, you will find `use` very handy.

Before:

```kantu
mod math;

let x = math.nat.Nat.S(math.nat.Nat.O);
```

After:

```kantu
mod math;

use math.nat.Nat;
use Nat.O;
use Nat.S;

let x = S(O);
```

#### `use` basic syntax

Just type something like `use foo.bar.baz;`. This will create a `baz` alias,
which will be an item of the current module.
Since it's an item of the current module, we don't need to use fully qualified
syntax, so we can simply write `baz` (instead of the full-blown `foo.bar.baz`) in the subsequent code.

#### Renaming

You can write `use foo.bar.baz as qux;` to make the alias `qux` (instead of `baz`). Example:

```kantu
use math.nat.Nat as Foo;

let plus = fun plus(-a: Foo, b: Foo): Foo {
    match a {
        .O => Foo.O,
        .S(a') => Foo.S(plus(a', b)),
    }
};
```

#### Wildcard imports

You can write `use foo.*;` as shorthand for aliasing all
the submodules of `foo`.
Only the submodules of `foo` will be aliases--other descendants
will not.

For example, if we have

`src/foo.ph`:

```
pub mod bar;
pub mod baz;

type Empty {}
```

`src/bar/mod.ph`:

```kantu
pub mod qux;
```

...then writing `use foo.*;` will be equivalent to writing

```kantu
use foo.bar;
use foo.baz;
```

Note that `foo.bar.qux` is NOT included, since even though it
is a descendant of `foo`, it is not a submodule of it.

Remember that name shadowing is forbidden!
Consequently, if a wildcard alias conflicts with an existing
module item of the same name, the compiler will emit an error.

### Ordering of item processing

Recall that Kantu forbids forward references.

In a single file project, it is trivial to see the order of module items--simply read the code from top-to-bottom.

However, with multiple files, a Kantu developer must understand
the order that the compiler processes module items, so they can
avoid forward references.

Fortunately, the rules is very straight forward.

#### Rules for determining processing order:

1. Start at the root module (i.e., `pack`).
   Keep processing items until you reach a submodule declaration
   (i.e., a `mod` statement).
2. Recursively use this algorithm to process that submodule's items.
3. Once you're done processing that submodule's items, go to step (1), and continue processing
   this module's items.

Here's an example to test your understanding.

#### Q: What order are the items processed in the example package below?

`src/mod.ph`:

```kantu
pub mod nat;
use nat.NaturalNumber as Nat;

pub mod plus;
```

`src/nat.ph`:

```kantu
pub type NaturalNumber {
    .O: NaturalNumber,
    .S(_: NaturalNumber): NaturalNumber,
}
```

`src/plus.ph`:

```kantu
use super.Nat;

pub let plus = fun plus(-a: Nat, b: Nat): Nat {
    match a {
        .O => Nat.O,
        .S(a') => Nat.S(plus(a', b)),
    }
};
```

#### A:

```text
pack.nat.NaturalNumber (type)
pack.nat (module)
pack.Nat (alias)
pack.plus.Nat (alias)
pack.plus.plus (constant)
pack.plus (mod)
```

### Constant transparency

Whether a constant declared with `let` is replaced with its
value during term evaluation is determined by the constant's
_transparency_.

By default, constants have module-level transparency, meaning
they will evaluate to their values in the defining module
(and descendant modules), but be normal forms in other modules.

This has significant implications for dependent types.

For example, is the following code is valid.

`src/mod.ph`:

```kantu
type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

type EqNat(a: Nat, b: Nat) {
    .Refl(c: Nat): EqNat(c, c),
}

let identity = fun _(T: Type, t: T): T {
    t
};
let ascribe = identity;

let my_number = Nat.O;

let my_number_equals_zero = identity(
    EqNat(my_number, Nat.O),
    EqNat.Refl(Nat.O),
);
```

The above code is valid because `my_number` is replaced
its value (i.e., `Nat.O`) during evaluation, therefore
`EqNat(my_number, Nat.O)` evaluates to the normal form
`EqNat(Nat.O, Nat.O)`, which is inhabited by the term
`Eqnat.Refl(Nat.O)`.

However, the following code is *in*valid.

`src/mod.ph`:

```kantu
type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

type EqNat(a: Nat, b: Nat) {
    .Refl(c: Nat): EqNat(c, c),
}

let identity = fun _(T: Type, t: T): T {
    t
};
let ascribe = identity;

mod my_constants;
use my_constants.my_number;

let my_number_equals_zero = identity(
    EqNat(my_number, Nat.O),
    EqNat.Refl(Nat.O),
);
```

`src/my_constants.ph`:

```kantu
use super.*;
pub let my_number = Nat.O;
```

This is because in this example, `my_number`'s transparency level
is `pack.my_constants`, so when evaluating the term
`EqNat(my_number, Nat.O)` in `pack`,
`my_number` is NOT replaced with `Nat.O`.
As a result, the `EqNat(my_number, Nat.O)` is its own normal form,
and `EqNat.Refl(Nat.O)` cannot be judged to inhabit it.

#### Modifying constant transparency

Write a module after the let. Here are some examples:

```kantu
// Global transparency
pub let(*) my_number_2 = Nat.O;

// Transparent in supermodule (and its descendants)
pub let(super) my_number_3 = Nat.O;

// Transparent in `pack.some.arbitrary.ancestor`
// (and its descendants)
pub let(pack.some.arbitrary.ancestor) my_number_4 = Nat.O;
```

You can write the same things inside the `()` used for specifying
transparency as you can inside the `pub()` used for specifying
visibility.
For example, `*`, `mod`, `super`, `super2`, `pack`, `pack.some.module`.

## Tamnyban (internal use only)

Tamnyban is a language for describing Kantu declarations.
There is no need for a Kantu user to learn it.

### Evaluation and typechecking

Evaluation (and therefore typechecking) is performed
from a given module's perspective.
The perspective determines what level of constant
transparency is required to substitute the constant.

### Name resolution

In Kantu code, a name starting with `pack`, `super`, or `mod`
is evaluated normally.
Any other name `x.y.z` is treated as shorthand for `mod.x.y.z`.

### `let` statements

```kantu[tamnyban]
pub(A) let(B) C.x = t;
```

Creates a constant `C.x`.

Requirements:

- `C` must be a module.
- `C.x` must not already exist.
- `B` must be a non-strict ancestor of `C`.
- `A` must be a non-strict ancestor of `B`.
- `t` must be well-typed when typechecked from the
  perspective of `B`.
- Every name in `t` must be visible to `B`.

In normal Kantu code, `C` is the module of the `let`
statement.
The user is allowed, but not required to
explicitly provide `A` and `B`.
If one or both are omitted, they default to the
most restrictive level (i.e., the deepest module)
that is legal.

Specifically:

- If `A` and `B` both omitted: `A, B := C`
- If `A` is omitted, `B` specified: `A := B`
- If `A` is specified, `B` omitted: `B := C`

### `type` statements

```kantu[tamnyban]
pub(A) type B.T(t1) { t2 }
```

Creates a type `B.T`.
Every variant `B.T.V_i` also has visibility `A`.

Requirements:

- `B` must be a module.
- `B.T` must not already exist.
- `A` must be a non-strict ancestor of `B`.
- Every name in `t1` and `t2` must be visible
  to `A`, and be well-typed from the perspective
  of `A`.

In normal Kantu code, `B` is the module of the
`type` statement.
The user is allowed, but not required to
explicitly provide `A`.
If `A` is omitted, then it defaults to `B`.

### `mod` statements

```kantu[tamnyban]
pub(A) mod B.M;
```

Creates a module `B.M`.

Requirements

- `B` must be a module.
- `B.M` must not already exist.
- `M` must be a single component.
- `A` must be a non-strict ancesstor of `B`.

In normal Kantu code, `B` is the module of the `mod` statement.
`A` may be explicitly specified or omitted.
If omitted, it defaults to `B`.

### `use` statements

#### Alias a constant

```kantu[tamnyban]
pub(A) use B.x as C.y&;
```

Creates alias `C.y& := B.x`.

Requirements:

- `C` must be a module.
- `C.y&` must not already exist.
- `B.x` must be visible to `C`.
- `A` must be a non-strict ancestor of `C`.

In normal Kantu code, `C` is the module of the `use` statement.
The user must explicitly specify `B`.
The user is allowed, but not required to
explicitly provide `A`.
If `A` is omitted, then it defaults to `C`.

#### Alias a type

```kantu[tamnyban]
pub(A) use B.T as C.U&;
```

Creates alias

```
C.U& := B.T
for V_i in B.T.dot_children:
    c.U.V_i& := B.T.V_i
```

Same requirements and defaults as constant aliases.

#### Alias a module

```kantu[tamnyban]
pub(A) use B as C.M&;
```

Creates alias

```
C.M& = B;
for x in B.dot_children:
    C.M.x& = B.x
```

Same requirements, plus

- `M` must be exactly one component long.

In normal Kantu code, `C` is the module of the `use` statement.
The user must explicitly specify `B` and `M`.
`A` may be specified or omitted (defaults to `C`).

#### Alias an alias

```kantu[tamnyban]
pub(A) use B& as C.M&;
```

Creates alias

```
C.M& = value(B&);
for x in B&.dot_children:
    C.M.x& = value(B&.x)
```

#### Wildcard `use` statements

```kantu[tamnyban]
pub(A) use B.* as C.*;
```

Creates alias

```
for x in B.dot_children {
    C.x := B.x
}
```

Requirements

- `C` and `B` must be a module
- `A` must be a non-strict ancestor of `C`.
- For all `x` in `B.dot_children`
  - `C.x` must not already exist.
  - `B.x` must be visible to `C`.
