//! This crate is for you if you have ever been annoyed at writing repetitive conditions like this
//! ```rust
//! # fn test(a:i32,x:i32,y:i32,z:i32) {
//! if x < a && y < a && z < a {
//! // ... do something
//! }
//! # }
//! ```
//! and wished you could replace that code by something more expressive and less repetitive. Now you can rewrite the code as
//! ```rust
//!# fn test(a:i32,x:i32,y:i32,z:i32) {
//! use fluent_comparisons::all_of;
//!
//! if all_of!({x,y,z} < a) {
//! // ... do something
//! }
//! # }
//! ```
//!
//! # Examples
//! The crate provides the macros `any_of`, `all_of` and `none_of` to facilitate writing expressive multicomparisons. The arguments
//! don't need to be numeric, but can be expressions of any type. Furthermore, a syntax for applying transformations to the set
//! on the left hand side is provided.
//!
//! ```
//! # use fluent_comparisons::{all_of,none_of,any_of};
//! // the following assertions hold
//! assert!(none_of!({1,2,3}>4));
//! assert!(any_of!({1,2,3}.map(|x|x%2)==0));
//! ```
//!
//! # Brief Description and Key Advantages
//!
//! In addition to providing an intuitive syntax, the macros compile to the same assembly as
//! the handwritten code ([check it on godbolt.org](https://godbolt.org/z/M3494a6Mc)).
//!
//! A further benefit is [lazy evaluation](https://doc.rust-lang.org/reference/expressions/operator-expr.html#lazy-boolean-operators) from
//! left to right as seen in the next snippet:
//!
//! ```rust
//! use fluent_comparisons::any_of;
//!
//! # fn cheap_calc(v : usize)-> usize {v}
//! # fn expensive_calc(v : usize)-> usize {v}
//! # fn test(arg1: usize, arg2:usize) {
//! // if cheap_calc(arg1) <=5, then the expensive calculation
//! // is never performed
//! let b = any_of!({cheap_calc(arg1), expensive_calc(arg2)}<=5);
//! // whereas if we did this, the expensive calculation would be
//! // performed regardless of the result of cheap_calc(arg1)
//! let b = [cheap_calc(arg1), expensive_calc(arg2)].iter().any(|val|val<=&5);
//! # }
//! ```
//!
//! And finally, you can rest assured in the warm and fuzzy feeling that this crate is excessively tested.
//!
//! ## Usage
//!
//! Refer to the items in the documentation below to learn more about the usage of the macros.

pub use fluent_comparisons_macros::any_of;

pub use fluent_comparisons_macros::all_of;

pub use fluent_comparisons_macros::none_of;

#[cfg(test)]
mod tests;
