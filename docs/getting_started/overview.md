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

type Color {
    .Rgb(r: U8, g: U8, b: U8): Color,
    .Hsv (h: U8, s: U8, v: U8): Color,
}

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

Types must be [strictly positive](https://cs.stackexchange.com/questions/55646/strict-positivity). TODO Give TLDR explanation.

In all likelihood, most users will not need to read the above article, because they fall into one of two categories:

1. Those who are type theory enthusiasts, and already know what strict positivity is. Therefore, reading the attached article is not necessary.
2. Those who come from more "mainstream" languages (e.g., Python, Java, JavaScript, TypeScript, Rust, C++, C, C#, Go, Ruby, Swift, etc.). These people will likely define types in a similar way to how they would in the other languages listed above. Those simple kinds of type definitions are almost always strictly positive. Therefore, reading the attached article is (probably) not necessary.

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
