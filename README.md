# fluent-comparisons
![build](https://github.com/geo-ant/fluent-comparisons/workflows/build/badge.svg?branch=main)
![tests](https://github.com/geo-ant/fluent-comparisons/workflows/tests/badge.svg?branch=main)
![lints](https://github.com/geo-ant/fluent-comparisons/workflows/lints/badge.svg?branch=main)

A crate offering a fluent syntax for multi-comparisons.

## Motivation

This crate is everyone for you if you have ever gotten annoyed at writing conditions like this

```rust
if x < a && y < a && z < a {
    // ... do something
}
``` 
and wished you could replace that code by something more expressive and less repetitive? If so, then this crate is for you, because it allows us to write the condition above as
```rust
if all_of!({x,y,z} < a) {
    // ... do something
}
``` 

## Brief Description and Key Advantages

The crate provides the macros `any_of`, `all_of` and `none_of` to facilitate writing conditions analogous to the example above. Additional to offering an intuitive syntax and more readable code, the macros compile down to the same assembly as the handwritten code for any nonzero optimization level ([check it on godbolt.org](https://godbolt.org/z/zTfTq6va5)).

A further benefit is [lazy evaluation](https://doc.rust-lang.org/reference/expressions/operator-expr.html#lazy-boolean-operators) from left to right as seen in the next snippet:

```rust
    // if cheap_calc(arg1) <=5, then the costly calculation
    // is never performed
    let cond = any_of!({cheap_calc(arg1), expensive_calc(arg2)}<=5);

    // whereas if we did this, the expensive calculation would be
    // performed regardless of the result of cheap_calculation(arg1)
    [cheap_calculation(arg1), expensive_calculation(arg2)].iter().any(|val|val<=5)
```

## Usage

The macros are called as `any_of!({/*list of expressions*/} operator rhs)`, where operator can be any of the binary comparison operators, i.e. `==`, `!=`, `<=`, `<`, `>`, and `>=`. The list of expressions on the left hand side is comma separated without a trailing comma. The right hand side is an expression as well.

The list of expressions can have a variadic number of elements but must have at least one. It must always be enclosed in curly braces. The expressions on the left hand side need not be of the same type, but the comparison with the right hand side must be valid. In particular, the expressions need not be numeric. 

The same goes for the `all_of` and `none_of` macros. Check the docs for more information.

## Links

This library is inspired by Björn Fahller's [DRY comparisons](https://github.com/rollbear/dry-comparisons) library, which I read about [in this blog post](https://www.fluentcpp.com/2020/01/03/dry-comparisons-a-c-library-to-shorten-redundant-if-statements/) on Jonathan Boccara's blog.