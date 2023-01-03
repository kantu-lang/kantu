# Yet Another Simple Config Language (YASCL)

Anyone should be able to learn in 90 seconds or less.

## Example

```yscl
kantu_version = "1.0.0"
dependencies = {
    foo = "2.0.3"
    bar = "bar"
    // Duplicate keys are permitted
    bar = "baz"
    lorem = {
        url = "https://github.com/kylejlin/nonexistent_repo"
    }
    // Omitting the spaces around the `=` is legal, but frowned upon.
    foo="bar"
}
licenses = [
    "MIT"
    "APACHE"
    {
        url = "https://github.com/kylejlin/nonexistent_repo/CUSTOM_LICENSE"
    }
]

// This is a comment.
foo = "bar" // You can also write comments at the end of a line.
```

## Rules

- 3 data types: atomic (string), list, and map
- String is the only atomic type
  - Enclose in double quotes
  - No newlines are allowed
  - Escapes:
    - `\\`: backslash (`\`)
    - `\"`: double quote (`"`)
    - `\n`: newline
    - `\uXXXX`: arbitrary Unicode character where `XXXX` is some four-digit hexadecimal value
- A map (`{}`) is an ordered sequence of key-value pairs
  - The key is an identifier
    - Identifier names can contain alphanumeric characters plus underscore
  - The value can be any YASCL value
  - An `=` separates the key from the value
  - The key and value must be on the same line
- A list (`[]`) is an ordered sequence of elements
  - Elements can be any YASCL value
- Map entries and list element rules
  - Separated by newline
  - One per line
- Files end with the `.yscl` extension
