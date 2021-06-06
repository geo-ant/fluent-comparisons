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

#[macro_export]
#[doc(hidden)]
/// # Internal Macro
/// This macro checks that the comparison operator in the expression is indeed allowed. If it is
/// allowed this macro evaluates to a unit/void statement. Otherwise it will give a compile error
/// explaining which operators are allowed
//#[macro_export]
#[doc(hidden)]
macro_rules! __check_operator {
    // these are the allowed comparison operators
    (==) => {};
    (<=) => {};
    (>=) => {};
    (!=) => {};
    (<) => {};
    (>) => {};
    // everything else is not allowed, including &&, ||, and such
    ($other:tt) => {
        std::compile_error!(
            "Illegal comparison operator. The only allowed operators are ==, !=, <=, >=, <, >"
        );
    };
}

/// Compare all values in a set to a common right hand side and decide whether the comparison returns `true` for *any of the values* in the set.
///
/// # Lazy Evaluation
///
/// If we write `any_of!({a,b}<c)`, this is equivalent to the hand coded `a<c && b<c`. That means that the comparisons are
/// evaluated [lazily](https://doc.rust-lang.org/reference/expressions/operator-expr.html#lazy-boolean-operators) from left to right. Once
/// the truth value of the expression can be determined, the evaluation stops. That means that e.g. for the expression `any_of!({1,some_func()}<5)`,
/// the function `some_func()` is not invoked.
///
/// # Usage
///
/// ## Basic Usage
///
/// For the basic use case we compare a set of values against a common right hand side. Invoke the macro using
/// `any_of!({/*list of expressions*/} operator rhs)`, where operator can be any of the binary comparison operators, i.e.
/// `==`, `!=`, `<=`, `<`, `>`, and `>=`. The list of expressions on the left hand side is comma separated without a trailing comma. The right hand side
/// is an expression as well.
///
/// The list of expressions can have a variadic number of elements but must have at least one. It must always be enclosed in
/// curly braces. The expressions on the left hand side need not be of the same type, but the comparison with the right hand side must be valid. In particular,
/// the expressions need not be numeric.
///
/// ```
/// # #[macro_use] extern crate fluent_comparisons; fn main() {
/// use rand::prelude::*;
/// // given:
/// let square = |val|val*val;
/// let v = vec![1, 2,3];
/// let mut rng = rand::thread_rng();
/// // the following assertions hold
/// assert!(any_of!({1,2,3}>2));
/// assert!(any_of!({4+4+1,square(7*2),120_i32.pow(2)}>8));
/// assert!(any_of!( {rng.gen::<usize>(),v.len(),2,1+1,"hello world".len()} == v.len()));
/// # }
/// ```
///
/// ## Usage with Transformations
///
/// We can also apply a transformation to the list on the left hand side before comparing to the right hand side.
/// For that, simply append `.map(...)` to the list and give an argument that transforms the values. The argument
/// to map can be any kind of invokable of a single argument, like a function or closure. Here the type requirements
/// are a bit stricter and all values on the left hand side must be of the same type.
///
/// ```
/// # #[macro_use] extern crate fluent_comparisons; fn main() {
/// // given
/// let square = |x|x*x;
/// // the following assertions hold
/// assert!(any_of!({4,square(2),2_i32.pow(2)}.map(|x|x+5)>8));
/// assert!(any_of!({4+1,3,5}.map(square)==9));
/// # }
/// ```
///
/// ## Usage with Predicates
///
/// This is a special case where the transformation maps to a boolean predicate. Instead of writing
/// `any_of!({...}.map(/*predicate f:x -> bool*/)==true)`, we can use the syntax `any_of!({...}.satisfy(/*predicate f:x -> bool*/))`,
/// which saves us the comparison with `true` on the right hand side. Don't use a predicate which
/// compares values with one of the comparison operators, because then you are better served with the
/// syntax above. Rather use it for more complex predicates:
///
/// ```
/// # #[macro_use] extern crate fluent_comparisons; fn main() {
/// fn is_prime_number(x:i32) -> bool {
///     /*some interesting math*/
/// # true
/// };
/// //this assertion holds
/// assert!(any_of!({12,14,5}.satisfy(is_prime_number)));
/// # }
/// ```
///
///
#[macro_export]
macro_rules! any_of {
    // variant with a predicate (does not use a comparison operator and rhs)
    ( {$($lh_sides:expr),+}.satisfy($($func:tt)+) ) => {
        any_of!({$($lh_sides),+}.map($($func)+)==true)
    };

    // variant with a transformation of the set
    ( {$($lh_sides:expr),+}.map($($func:tt)+) $operator:tt $rhs:expr) => {
        {
            $crate::__check_operator!($operator);
            //by fixing this here, we have more type deduction powers but also less
            //flexibility in generic arguments. We could also pass the expanded tt func to a single
            //tt in a submacro (by putting (...) around it) and then use that function, which is more
            //powerful when passing generic functions, but less intuitive when passing lambdas
            //so this map is more akin to a map in a collection. The other is more akin to a C++
            //transform of a heterogeneous collection. For that we might want to pass a path or ident
            //instead of the token tree. Because the token tree is just a trick to get lambdas
            //as well. But since the other way isn't great for lambdas anyway we can just skip it.
            let map_func = $($func)+;
            $( (map_func($lh_sides) $operator $rhs) )||+
        }
    };

    //variant without map (requires a comparison operator and rhs)
    ( {$($lh_sides:expr),+} $operator:tt $rhs:expr)=> {
        {
            $crate::__check_operator!($operator);
            $( ($lh_sides $operator $rhs) )||+
        }
    };
}

/// Compare all values in a set to a common right hand side and decide whether the comparison returns `true` for *all of the values* in the set.
///
/// # Usage
/// The usage is analogous to the [any_of](crate::any_of) macro and is documented in more detail there.
/// Just like `any_of`, this macro also performs lazy evaluation.
///
/// ## Examples
/// The following examples show how to use the macro.
/// ```
/// # #[macro_use] extern crate fluent_comparisons; fn main() {
/// let square = |val|val*val;
/// // the following assertion holds
/// assert!(all_of!({4+4+1,square(7*2),120_i32.pow(2)}>0));
///
/// let v = vec![1, 2,3,4,5];
/// // the following assertion holds
/// assert!(all_of!( {square(2),v.len() as i32,2,1+1,"hello".len() as i32} <= v.len() as i32));
/// # }
/// ```
#[macro_export]
macro_rules! all_of {

    ( {$($lh_sides:expr),+}.satisfy($($func:tt)+) ) => {
        all_of!({$($lh_sides),+}.map($($func)+)==true)
    };

    ( {$($lh_sides:expr),+}.map($($func:tt)+) $operator:tt $rhs:expr) => {
        {
            $crate::__check_operator!($operator);
            let map_func = $($func)+;
            $( (map_func($lh_sides) $operator $rhs) )&&+
        }
    };

    ( {$($lh_sides:expr),+} $operator:tt $rhs:expr)=> {
        {
            $crate::__check_operator!($operator);
            $( ($lh_sides $operator $rhs) )&&+
        }
    };
}

/// Compare all values in a set to a common right hand side and decide whether the comparison returns `true` for *none of the values* in the set.
///
/// # Usage
/// The usage is analogous to the [any_of](crate::any_of) macro and is documented in more detail there.
/// Just like `any_of`, this macro also performs lazy evaluation.
///
/// ## Examples
/// The following examples show how to use the macro.
/// ```
/// # #[macro_use] extern crate fluent_comparisons; fn main() {
///
/// let square = |val|val*val;
/// // the following assertion holds
/// assert!(none_of!({4+4+1,square(7*2),120_i32.pow(2)}<0));
///
/// let v = vec![1, 2,3,4,5];
/// // the following assertion holds
/// assert!(none_of!( {square(2),v.len() as i32,2,1+1,"hello".len() as i32} > v.len() as i32));
/// # }
/// ```
#[macro_export]
macro_rules! none_of {
    ( {$($lh_sides:expr),+}.satisfy($($func:tt)+) ) => {
        none_of!({$($lh_sides),+}.map($($func)+)==true)
    };

    ( {$($lh_sides:expr),+}.map($($func:tt)+) $operator:tt $rhs:expr) => {
        {
            $crate::__check_operator!($operator);
            let map_func = $($func)+;
            $( !(map_func($lh_sides) $operator $rhs) )&&+
        }
    };

    ( {$($lh_sides:expr),+} $operator:tt $rhs:expr)=> {
        {
            $crate::__check_operator!($operator);
            $( !($lh_sides $operator $rhs) )&&+
        }
    };
}

//TODO DOCUMENT
pub use fluent_comparisons_macros::exactly;

#[cfg(test)]
mod tests;
