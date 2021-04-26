///! This crate contains the macros for the fluent-comparisons crate

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
            "This operator is not allowed. The only allowed operators are ==, !=, <=, >=, <, >"
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
/// # use fluent_comparisons_macros::any_of;
/// use rand::prelude::*;
/// // given:
/// let square = |val|val*val;
/// let v = vec![1, 2,3];
/// let mut rng = rand::thread_rng();
/// // the following assertions hold
/// assert!(any_of!({1,2,3}>2));
/// assert!(any_of!({4+4+1,square(7*2),120_i32.pow(2)}>8));
/// assert!(any_of!( {rng.gen::<usize>(),v.len(),2,1+1,"hello world".len()} == v.len()));
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
/// # use fluent_comparisons_macros::any_of;
/// // given
/// let square = |x|x*x;
/// // the following assertions hold
/// assert!(any_of!({4,square(2),2_i32.pow(2)}.map(|x|x+5)>8));
/// assert!(any_of!({4+1,3,5}.map(square)==9));
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
/// # use fluent_comparisons_macros::any_of;
/// fn is_prime_number(x:i32) -> bool {
///     /*some interesting math*/
/// # true
/// };
/// //this assertion holds
/// assert!(any_of!({12,14,5}.satisfy(is_prime_number)));
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
/// # use fluent_comparisons_macros::all_of;
///
/// let square = |val|val*val;
/// // the following assertion holds
/// assert!(all_of!({4+4+1,square(7*2),120_i32.pow(2)}>0));
///
/// let v = vec![1, 2,3,4,5];
/// // the following assertion holds
/// assert!(all_of!( {square(2),v.len() as i32,2,1+1,"hello".len() as i32} <= v.len() as i32));
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
/// # use fluent_comparisons_macros::none_of;
///
/// let square = |val|val*val;
/// // the following assertion holds
/// assert!(none_of!({4+4+1,square(7*2),120_i32.pow(2)}<0));
///
/// let v = vec![1, 2,3,4,5];
/// // the following assertion holds
/// assert!(none_of!( {square(2),v.len() as i32,2,1+1,"hello".len() as i32} > v.len() as i32));
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

// TODO FINISH THIS UP, TEST IT AND DOCUMENT IT
// TODO: make this as simple as the ones above w/o recursion
// TODO maybe make the syntax exactly!( 12 of {a,b,...} <= 5) possible. Maybe we can even allow
// an ident instead of the literal. We probably can't allow an expression, but that is fine...
// #[macro_export]
// macro_rules! exactly_one_of {
//     //TODO CAUTION: THIS COULD BE CALLED WITH ONE ARGUMENT. MAKE SURE THAT THIS PRODUCES A VALID RESULT
//     // expression like any_of!( {1,v.len(),4} < 3)
//     ( {$($lh_sides:expr),+} $operator:tt $rhs:expr)=> {
//         {
//             $crate::__check_operator!($operator);
//             1u32 == exactly_one_of!(@internal lhs={$($lh_sides),+}, op=[$operator], rhs = $rhs, expanded = {0u32})
//         }
//     };
//
//     // internal rules, recursion final case
//     (@internal lhs = {$head:expr},op = [$op:tt], rhs = $rhs:expr, expanded = {$expanded:expr}) => {
//         $expanded + {if $head $op $rhs {1u32}else{0u32}}
//     };
//
//     // internal rules, recursion case
//     (@internal lhs = {$head:expr, $($tail:expr),*}, op = [$op:tt], rhs = $rhs:expr, expanded = {$expanded:expr}) =>{
//         exactly_one_of!(@internal lhs={$($tail),*}, op=[$op], rhs = $rhs, expanded = {$expanded + {if $head $op $rhs {1u32}else{0u32}}})
//     };
// }
