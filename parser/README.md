Toy Expression Parser
=====================

A very basic recursive-decent parser for arithmetic expressions based on [blog post] and [git repo] from [Adrian Neumann]

Changes
-------

1. Supports subtraction and division
2. More detailed output formats
3. Expression evaluation
4. Parser bookkeeping includes a bit more information
5. Grouping normalization

Limitations
-----------

TL;DR: It's a toy, don't use this as a calculator.

It's a right-biased parser, so `1/2/4` is parsed as `1/(2/4)` instead of `(1/2)/4`.

This didn't matter to the original post because addition and multiplication are associative.

|   input | left-biased expr | right-biased expr | left-biased result | right-biased result |
| ------: | ---------------: | ----------------: | -----------------: | ------------------: |
| `1+2+4` |        `(1+2)+4` |         `1+(2+4)` |                  7 |                   7 |
| `1-2-4` |        `(1-2)-4` |         `1-(2-4)` |                 -5 |                   3 |
| `1*2*4` |        `(1*2)*4` |         `1*(2*4)` |                  8 |                   8 |
| `1/2/4` |        `(1/2)/4` |         `1/(2/4)` |                1/8 |                   2 |

[blog post]: https://adriann.github.io/rust_parser.html
[git repo]: https://github.com/adrianN/simple_rust_parser
[Adrian Neumann]: https://github.com/adrianN