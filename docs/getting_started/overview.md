# Pamlihu overview

## Identifiers

Identifier names can contain the following characters:

- Unicode letters
- Unicode numbers
- Unicode punctuation
- Unicode symbols

...with the exception of:

- The characters `;:,.@=-?()[]{}<>` cannot appear anywhere.
- The characters `0123456789` cannot appear as the first character, but may appear everywhere else.
- White space cannot appear anywhere.

Additionally, the following strings are reserved words, and cannot be used as identifiers:

```
_ (the underscore)
∀ (Unicode universal quantifier symbol)
∃ (Unicode existential quantifier symbol)

type
Type
Type0
Type1
Type2
Type3
let
fun
match
forall
exists
check
goal

struct
var
trait

pub
priv
mod
package
use
namespace

extern
unsafe
async

notation
```

## Type Definitions

Some examples:

```pamlihu
type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

type Bool {
    .True: Bool,
    .False: Bool,
}

type False {}

type List(T: Type) {
    .Nil(T: Type): List(T),
    .Cons(T: Type, car: T, cdr: List(T)): List(T),
}

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

// Dependent types 🎉

type LessThanOrEqualTo(L: Nat, R: Nat) {
    .Equal(n: Nat): LessThanOrEqualTo(n, n),
    .Step(a: Nat, b: Nat, H: LessThanOrEqualTo(a, b)): LessThanOrEqualTo(a, Nat.S(b)),
}

type ListOfEvenNats {
    .C(l: List(Nat), H_all_even: forall(n: Nat, H_in: In(Nat, n, l)) { Even(n) }): ListOfEvenNats,
}
```

Note that empty parameter lists are always omitted (so there is never `()`, outside of invocations).

### Type Restrictions

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

## `let` Aliases

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

## `match` Expressions

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

## Functions

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

It is strongly encouraged to make non-recursive functions anonymous
if you're assigning them to a `let` alias or passing them as a labeled argument
to a function call, since the other name (i.e., `let` binding or argument label, respectively)
should already provide sufficient documentation for readers of the code, and an extra
name in the `fun` expression just becomes clutter.

The main purpose of allowing `fun` expressions to be named is to allow recursion.

TODO: Recursion section

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
check
    // Type assertions:
    expr0: Type0,
    expr1: Type1,
    /* ... */,
    // Normal form assertions:
    expr'0 = expr''0,
    expr'1 = expr''1,
    /* ... */
{
    output_expression
}
```

As you can see, you can pass in zero or more _type assertions_ and zero or more
_normal form assertions_.
There must be at least one total assertions (i.e., `check { output_expression }` is illegal).

Semantically, a `check` expression immediately evaluates to its output (which is
why they have no runtime impact).

However, what is useful is that the compiler will produce warnings if any of the
assertions are incorrect.
Furthermore, a good compiler will generate corrections for the incorrect types/values.
This way, we can use `check` expressions to debug confusing type errors.

## Value definitions

Examples:

```pamlihu
let q = Quaternion.C(1, 1, 1, 1);
let red = Color.Rgb(255, 0, 0);
let S: Nat.S;
let _3 = S(S(S(Nat.O)));

let plus = fun plus_(-a: Nat, b: Nat): Nat {
    match a {
        .O => b,
        .S(a') => Nat.S(plus_(a', b)),
    }
};
let mult = fun mult_(-a: Nat, b: Nat): Nat {
    match a {
        .O => Nat.O,
        .S(a') => plus(b, mult_(a', b)),
    }
};
let fact = fun fact_(-a: Nat): Nat {
    match a {
        .O => Nat.S(Nat.O),
        .S(a') => mult(a, fact_(a')),
    }
};

let pred = fun _(n: Nat): Nat {
    match n {
        .O => Nat.O,
        .S(n') => n',
    }
};
let subtract = fun substract_(min: Nat, -sub: Nat): Nat {
    match sub {
        .O => min,
        .S(sub') => pred(subtract_(min, sub')),
    }
};

let _4 = S(S(S(S(Nat.O))));
let _1 = sub(_4, _3);
```

### Recursion Restrictions

To prevent infinite recursion,
Pamlihu enforces to two restrictions:

1. **No forward references except for recursive `fun` calls**

   This implies that mutal recursion is forbidden. For example:

   **Forbidden:**

   ```pamlihu
   let f = fun _(n: Nat): Nat {
        match n {
            .O => Nat.O,
            // Forbidden because `g` is not yet defined
            .S(n') => g(n'),
        }
   };

   let g = fun _(n: Nat): Nat {
        match n {
            .O => Nat.O,
            .S(n') => f(n'),
        }
   };
   ```

   However, recursive fun calls are permitted. For example:

   **Allowed:**

   ```pamlihu
   let always_returns_zero = fun zero_(-n: Nat): Nat {
        match n {
            .O => Nat.O,
            .S(n') => zero_(n'),
        }
   };
   ```

2. **Recursive functions must have exactly one explicitly decreasing parameter**

   Every recursive function must have exactly one parameter annotated with `-` that
   indicates that it is a _decreasing parameter_.
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

##
