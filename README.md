# fluent-comparisons
![build](https://github.com/geo-ant/fluent-comparisons/workflows/build/badge.svg?branch=main)
![tests](https://github.com/geo-ant/fluent-comparisons/workflows/tests/badge.svg?branch=main)
![lints](https://github.com/geo-ant/fluent-comparisons/workflows/lints/badge.svg?branch=main)

**Fluent syntax for multi-comparisons.**

## Motivation

This crate is for you if you have ever gotten annoyed at writing repetitive conditions like this

```rust
if x < a && y < a && z < a {
    // ... do something
}
``` 
and wished you could replace that code by something more expressive and less repetitive? Now you can write the condition above as
```rust
use fluent_comparisons::all_of;

if all_of!({x,y,z} < a) {
    // ... do something
}
``` 

## Brief Description and Key Advantages

The crate provides the macros `any_of`, `all_of` and `none_of` to facilitate writing expressive multicomparisons. In addition to providing an intuitive syntax, the macros compile to the same assembly as the handwritten code ([check it on godbolt.org](https://godbolt.org/z/M3494a6Mc)).

A further benefit is [lazy evaluation](https://doc.rust-lang.org/reference/expressions/operator-expr.html#lazy-boolean-operators) from left to right as seen in the next snippet:

```rust
    // if cheap_calc(arg1) <=5, then the expensive calculation
    // is never performed
    any_of!({cheap_calc(arg1), expensive_calc(arg2)}<=5)

    // whereas if we did this, the expensive calculation would be
    // performed regardless of the result of cheap_calculation(arg1)
    [cheap_calc(arg1), expensive_calc(arg2)].iter().any(|val|val<=&5)
```

## Usage

Use the macros by writing `any_of!({/*list of expressions*/} operator rhs)`, where operator can be any of the binary comparison operators, i.e. `==`, `!=`, `<=`, `<`, `>`, and `>=`. The list of expressions on the left hand side is comma separated without a trailing comma. The right hand side is an expression as well.

The list of expressions can have a variadic number of elements but must have at least one. It must always be enclosed in curly braces. The expressions on the left hand side need not be of the same type, but the comparison with the right hand side must be valid individually. Furthermore, the expressions can evaluate to anything that can be compared to `rhs`, not just numbers. 

The same goes for the `all_of` and `none_of` macros. Check the docs for more information.

## Links

This library is inspired by Björn Fahller's [DRY comparisons](https://github.com/rollbear/dry-comparisons) Modern C++ library, which I read about [in this blog post](https://www.fluentcpp.com/2020/01/03/dry-comparisons-a-c-library-to-shorten-redundant-if-statements/) on Jonathan Boccara's blog.