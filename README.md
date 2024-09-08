# `rfmt`

_R parsing, linting and formatting_

## Try it out

For now, we just handle the parsing of code into tokens. These tokens are 
only what is emitted by the parser and might not map conveniently into R-like
_S-Expressions_.

Future work may map these into a more R-like representation of the syntax tree
to make it more paletable to work with for R folks.

```sh
cargo run --example parsing
```

For now, you'll see a deeply nested syntax tree of parsing rules. It will 
look something like this:

```
Ok(
    [
        Pair {
            rule: expr,
            span: Span {
                str: "f <- function(a, b = 2, c = { expression }) {\n    if (a + b) {\n        \"hello, world\"  # return something\n    } else {\n        \"fizzbuzz\"      # or return something else\n    }\n}",
                start: 122,
                end: 299,
            },
            inner: [
                Pair {
                    rule: prefixed,
                    span: Span {
                        str: "f ",
                        start: 122,
                        end: 124,
                    }
```

And continues far longer than you probably care to scroll!
