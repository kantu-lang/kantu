# Pamlihu overview

## Identifiers

Identifier names may contain letters, underscores, and digits.
Identifier names may not begin with a digit.

To clarify: By "letters", we are referring to the 52 English letters (a-z and A-Z).
By "digits", we are referring to the 10 Arabic numerals (0-9).

In the future, we may allow identifiers to contain more characters,
such as letters with diacritics (e.g., "å")
and characters from different languages (e.g., "あ", "四", "한").
However, this is currently unsupported.

The following strings are reserved words, and cannot be used as identifiers:

```
_ (the underscore)

type
struct
let
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
    .rgb { r: U8, g: U8, b: U8 }: Color,
    .hsv { h: U8, s: U8, v: U8 }: Color,
}

type Vec2 {
    .vec2(Nat, Nat) : Vec2,
}

type Quaternion {
    .quaternion(one: Real, i: Real, j: Real, k: Real),
}

type Option(Type) {
    .none(T: Type): Option(T),
    .some(T: Type, v: T): Option(T),
}

type List(Type) {
    .nil(T: Type): List(T),
    .cons(T: Type, car: T, cdr: List(T)): List(T),
}

// Dependent types 🎉

type EvenNatList {
    .even_nat_list(
        l: list(Nat),
        H: forall(n: Nat) { l.contains(n) = Bool.true },
    ): EvenNatList,
}

type LessThanOrEqualTo(Nat, Nat) {
    .equal(n: Nat): LessThanOrEqualTo(n, n),
    .step(a: Nat, b: Nat, H: LessThanOrEqualTo(a, b)): LessThanOrEqualTo(a, Nat.successor(b)),
}
```

### Variant Parameters

As you can see, each type has zero or more _variants_.
Some examples of variants include `Bool.true`, `Color.rgb { r: U8, g: U8, b: U8 }`, and `List.nil(T: Type)`.

Each variant takes zero or more _parameters_.
Parameter set be defined in 3 ways:

1. Unit-like: We call an empty parameter set (i.e., one with zero parameters) a _unit-like_ parameter set.

   Example: `Bool.true`.

2. Tuple-like: Parameter sets with parentheses are called tuple-like.
   With tuple-like parameter sets, **names are optional but order matters**.

   Within the parentheses, the parameters may either be _named_
   (e.g., `Quaternion.quaternion(one: Real, i: Real, j: Real, k: Real)`) or _unnamed_
   (e.g., `Vec2.vec2(Nat, Nat)`).

   A given parameter set must either contain only named parameters, or
   only unnamed parameters--it cannot contain a mix of both types of parameters.

   When providing arguments, you do not write any parameter names, even
   if the parameter set is named.

   For example:

   Right: `let q = Quaternion.quaternion(1, 1, 1, 1);`.

   Wrong: `let q = Quaternion.quaternion(one: 1, i: 1, j: 1, k: 1);`.

3. Record-like: Parameter sets with curly braces are called record-like.
   With record-like parameter sets, **names are required, but order does not matter**.

   Examples: `Color.rgb { r: U8, g: U8, b: U8 }`, `Color.hsv { h: U8, s: U8, v: U8 }`.

   As previously stated, when providing arguments, you _must_ write parameter names.

   For example:

   Right: `let red = Color.rgb { r: 255, g: 0, b: 0 };`

   Wrong: `let red = Color.rgb { 255, 0, 0 };`

   Since order doesn't matter, writing
   `let red = Color.rgb { b: 0, g: 0, r: 255 };` would
   have also been equivalent.

### Type Parameters

Types can also take parameters. For example, `Option` takes one (unnamed) parameter of type `Type`.

> `Type` is the type of all other types. However, `Type` is not a member of itself. In the future, we may add higher-order universes.

As another example, LessThanOrEqualTo takes two (unnamed) parameters, both of
type `Type`.

Type parameter sets can be unit-like, tuple-like, or record-like, just like
variant parameter sets can be.

### Ergonomics and syntactic sugar

1.  If a type has zero parameters, you don't need to declare the type of each
    variant.

    For example, in the above example

    ```pamlihu
    type Bool {
        .true: Bool,
        .false: Bool,
    }
    ```

    could be rewritten to

    ```pamlihu
    type Bool {
        .true,
        .false,
    }
    ```

    since `Bool` has zero parameters.

2.  If a type has exactly one variant, you are allowed to (and encouraged) to
    use `_` as the variant name.

    For example, in the above example

    ```pamlihu
    type Quaternion {
        .quaternion(one: Real, i: Real, j: Real, k: Real),
    }
    ```

    should be rewritten to

    ```pamlihu
    type Quaternion {
        ._(one: Real, i: Real, j: Real, k: Real),
    }
    ```

    The rationale behind this convention is that it's redundant to
    specify the variant type if there is only one, so the variant
    name should be as short as possible, to minimize code verbosity.

    Note that `_` is only permitted as a variant name in types with
    exactly one variant.

## Value definitions

Examples:

```pamlihu
let q = Quaternion._(1, 1, 1, 1);
let red = Color.rgb { r: 255, g: 0, b: 0 };
let three = Nat.successor(Nat.successor(Nat.successor(Nat.zero)));

let add(a: Nat, b: Nat) = match a {
    .zero => b,
    .successor a_pred => Nat.successor(add(a_pred, b)),
};
let mult(a: Nat, b: Nat) = match a {
    .zero => Nat.zero,
    .successor a_pred => add(b, mult(a_pred, b)),
};
let fact(n: Nat) = match n {
    .zero => Nat.successor(Nat.zero),
    .successor n_pred => mult(n, fact(n_pred)),
};

let pred(n: Nat) = match n {
    .zero => Nat.zero,
    .successor n_pred => n_pred,
};

let sub { min: Nat, sub: Nat } = match sub {
    .zero => min,
    .successor sub_pred => pred(sub(min, sub_pred)),
};

let s: Nat.successor;
let four = s(s(s(s(Nat.zero))));
let one = sub { min: four, sub: three };
```

Every value has zero or more parameters.
Values with zero parameters are called _constants_.
Values with one or more parameters are called _functions_.

A parameter set may be unit-like, tuple-like, or record-like, just like
variant parameter sets can be.
From the definition of "constant", it follows that constants
always have unit-like parameters sets.
Similarly, from definition of "function", it follows that functions must
have tuple-like or record-like parameter sets.

Values are not permitted to have unnamed tuple-like parameter sets.

## Notation

Example:

```pamlihu
notation (a: Nat) "+" (b: Nat) = plus(a, b);
notation (a: Nat) "+" (b: Nat) = mult(a, b);
notation (n: Nat) "!" = fact(n);

let n = (a + (b * c))!;
let m = (a + b) + c;
```

Pamlihu does not support implicit operator precedence--the order of
operations must be explicitly written out with parentheses.

However, you are allowed to specify left or right associativity.

For example:

```pamlihu
notation {
    in: (a: Nat) "+" (b: Nat);
    out: plus(a, b);
    associativity: left;
}

// Look ma, no parentheses!
let m = a + b + c;
```

You can also declare notation that only applies in certain contexts:

```pamlihu
notation {
    context: {
        left_delimiter: "[" (T: Type) "{";
        right_delimiter: "]";
    }
    in: (left: T) "," (right: List(T));
    out: List.cons(T, left, right);
    associativity: right;
}

notation {
    context: {
        left_delimiter: "[" (T: Type) "{";
        right_delimiter: "]";
    }
    in: "}";
    out: List.nil(T);
}

let x = [Bool {Bool.true,Bool.false,Bool.true,}];
let y = List.cons(Bool, Bool.true, List.cons(Bool, Bool.false, List.cons(Bool, Bool.true, List.nil(Bool))));
// x = y
```

TODO Symbol rules

TODO clarify let = and =?
