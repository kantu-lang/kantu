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
type False {}

type Bool {
    .true: Bool,
    .false: Bool,
}

type Color {
    .rgb(r: U8, g: U8, b: U8): Color,
    .hsv (h: U8, s: U8, v: U8): Color,
}

type Vec2 {
    .vec2(Nat, Nat) : Vec2,
}

type Quaternion {
    .quaternion(one: Real, i: Real, j: Real, k: Real),
}

type Option(T: Type) {
    .none(T: Type): Option(T),
    .some(T: Type, v: T): Option(T),
}

type List(T: Type) {
    .nil(T: Type): List(T),
    .cons(T: Type, car: T, cdr: List(T)): List(T),
}

type Equal(T: Type, x: T, y: T) {
    .refl(T: Type, x: T): Equal(T, x, x),
}

type Or(L: Type, R: Type) {
    .inl(L: Type, R: Type, l: L): Or(L, R),
    .inr(L: Type, R: Type, r: R): Or(L, R),
}

type False {}

let In = fun In(T: Type, item: T, list: List(T)): Type => match list {
    List.nil(_T) => False,
    List.cons(_T, car, cdr) => Or(Equal(T, item, car), In(T, item, cdr)),
};

// Dependent types 🎉

type LessThanOrEqualTo(L: Nat, R: Nat) {
    .equal(n: Nat): LessThanOrEqualTo(n, n),
    .step(a: Nat, b: Nat, H: LessThanOrEqualTo(a, b)): LessThanOrEqualTo(a, Nat.successor(b)),
}

type ListOfEvenNats {
    .list_of_even_nats(l: List(Nat), H_all_even: forall(n: Nat, H_in: In(Nat, n, l)) => Even(n)): ListOfEvenNats,
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
let q = Quaternion.quaternion(1, 1, 1, 1);
let red = Color.rgb(r: 255, g: 0, b: 0);
let three = Nat.successor(Nat.successor(Nat.successor(Nat.zero)));

let add = fun add(a: Nat, b: Nat): Nat => match a {
    .zero => b,
    .successor a_pred => Nat.successor(add(a_pred, b)),
};
let mult = fun mult(a: Nat, b: Nat): Nat => match a {
    .zero => Nat.zero,
    .successor a_pred => add(b, mult(a_pred, b)),
};
let fact = fun fact(n: Nat): Nat => match n {
    .zero => Nat.successor(Nat.zero),
    .successor n_pred => mult(n, fact(n_pred)),
};

let pred = fun pred(n: Nat): Nat => match n {
    .zero => Nat.zero,
    .successor n_pred => n_pred,
};

let subtract = fun subtract(min: Nat, sub: Nat): Nat => match sub {
    .zero => min,
    .successor sub_pred => pred(subtract(min, sub_pred)),
};

let s: Nat.successor;
let four = s(s(s(s(Nat.zero))));
let one = sub(min: four, sub: three);
```

### Recursion Restrictions

To prevent infinite recursion,
Pamlihu forbidens mutual recursion.
In other words, the only function a
given function can recursively call is itself.

For example:

**Forbidden:**

```pamlihu
let f(n: Nat) = match n {
    .zero => Nat.zero,
    .successor n_pred => g(n_pred),
};

let g(n: Nat) = match n {
    .zero => Nat.zero,
    .successor n_pred => f(n_pred),
};
```

This above example is forbidden because `f` and `g` mutually refer to
each other.

**Permitted:**

```pamlihu
let useless_function(n: Nat) = match n {
    .zero => Nat.zero,
    .successor n_pred => useless_function(n),
};
```

The above example is permitted, because the only function `f`
recursively calls is `f` itself.

2. For any recursive call, the arguments must be all identical, with the
   following exceptions:

   1. At least one argument must be a syntactic substructure of the value
      the value that was passed in. TODO: Clarify this.
