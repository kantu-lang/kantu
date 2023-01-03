# Yet Another Simple Config Language (YSCL)

YSCL (rhymes with "rascal") is a general purpose config language that
aims to be so simple that anyone can go
from zero to absolute mastery in less than five minutes.

## Learn by example

```yscl
// Hi, I'm a comment! I start with two slashes.
// I am ignored by the parser.
// I must go at the beginning of a line
       // (leading whitespace is permitted, however).

// This is an entry. Every file has zero or more of them.
kantu_version = "1.0.0"

// Every entry has a _key_ and a _value_
//
// A key is one or more (ASCII) letters, digits, or underscores.
//
// The value can be...
//
// 1. A string, like you saw above ("1.0.0")
// 2. A map (explained later)
// 3. A list (explained later)

// This is a map.
// A map is a sequence of entries enclosed in curly braces (`{}`)
dependencies = {
    foo = "2.0.3"
    bar = "bar"
    // Duplicate keys are permitted
    bar = "baz"
    lorem = {
        url = "https://github.com/kylejlin/nonexistent_repo"
    }

    // Note: There can only be one entry per line.
}

// This is a list.
// A list is a sequence of values enclosed in square brackets (`[]`)
licenses = [
    "MIT"
    "APACHE"
    {
        url = "https://github.com/kylejlin/nonexistent_repo/CUSTOM_LICENSE"
    }

    // Note: There can only be one value per line.
]

// There are 4 supported string escape sequences:
sequences = [
  // Double quote
  "\""

  // Backslash
  "\\"

  // Newline
  "\n"

  // Arbitrary Unicode codepoint
  "\u263A"
  // You can replace `263A` with any 4 hexadecimal characters.
]
```

## More details

- Files end with the `.yscl` extension
- The file is implicitly a map

  - The top-level `{}` are implicit, and must **NOT** be included.

    For example, the following is illegal:

    ```yscl
    // WRONG - Do not copy!
    {
        foo = "bar"
        lorem = {
            ipsum = "dolor"
        }
    }
    ```

    The following is correct:

    ```yscl
    foo = "bar"
    lorem = {
        ipsum = "dolor"
    }
    ```
