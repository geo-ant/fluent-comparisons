# fluent-comparisons

![build](https://github.com/geo-ant/fluent-comparisons/workflows/build/badge.svg?branch=main)
![tests](https://github.com/geo-ant/fluent-comparisons/workflows/tests/badge.svg?branch=main)
![lints](https://github.com/geo-ant/fluent-comparisons/workflows/lints/badge.svg?branch=main)
[![Coverage Status](https://coveralls.io/repos/github/geo-ant/fluent-comparisons/badge.svg?branch=main)](https://coveralls.io/github/geo-ant/fluent-comparisons?branch=main)
![maintenance-status](https://img.shields.io/badge/maintenance-passively--maintained-yellowgreen.svg)

**Fluent syntax for multi-comparisons.**

This crate is for you if you have ever been annoyed at writing repetitive conditions like this
```rust
if x < a && y < a && z < a {
    // ... do something
}
```
and wished you could replace that code by something more expressive and less repetitive. Now you can rewrite the code as
```rust
use fluent_comparisons::all_of;
if all_of!({x,y,z} < a) {
    // ... do something
}
```

## Examples
The crate provides the macros `any_of`, `all_of` and `none_of` to facilitate writing expressive multicomparisons. The arguments
don't need to be numeric, but can be expressions of any type. Furthermore, a syntax for applying transformations and predicates to the set
on the left hand side is provided.
```rust
// the following assertions hold
assert!(none_of!({1,2,3}>4));
assert!(any_of!({1,2,3}.map(|x|x%2)==0));
assert!(all_of!({2,5,7}.satisfy(is_prime_number)));
```

## Brief Description and Key Advantages
In addition to providing an intuitive syntax, the macros compile to the same assembly as 
the handwritten code ([check it on godbolt.org](https://godbolt.org/z/M3494a6Mc)). 
A further benefit is [lazy evaluation](https://doc.rust-lang.org/reference/expressions/operator-expr.html#lazy-boolean-operators) from
left to right as seen in the next snippet:
```rust
use fluent_comparisons::any_of;
// if cheap_calc(arg1) <=5, then the expensive calculation
// is never performed
let b = any_of!({cheap_calc(arg1), expensive_calc(arg2)}<=5);
// whereas if we did the following, the expensive calculation would be
// performed regardless of the result of cheap_calc(arg1)
let b = [cheap_calc(arg1), expensive_calc(arg2)].iter().any(|val|val<=&5);
```

And finally, you can rest assured in the warm and fuzzy feeling that this crate is excessively tested.

## Usage

Use the macros by writing `any_of!({/*list of expressions*/} operator rhs)`, 
where operator can be any of the binary comparison operators, 
i.e. `==`, `!=`, `<=`, `<`, `>`, and `>=`. The list of expressions on the left hand side 
is comma separated. The right hand side of the comparison is an expression as well.

The list of expressions can have a variadic number of elements but must have at least one.
It must always be enclosed in curly braces. The expressions on the left hand side need not be 
of the same type, but the comparison with the right hand side must be valid individually. 
Furthermore, the expressions can evaluate to anything that can be compared to `rhs`, not just numbers. 

The same goes for the `all_of` and `none_of` macros. Check the docs for more information.

## Links

This library is inspired by Björn Fahller's [DRY comparisons](https://github.com/rollbear/dry-comparisons) Modern C++ library, which I read about [in this blog post](https://www.fluentcpp.com/2020/01/03/dry-comparisons-a-c-library-to-shorten-redundant-if-statements/) on Jonathan Boccara's blog.
