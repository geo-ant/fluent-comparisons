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
    // these are the allowed operators
    (==) => {};
    (<=) => {};
    (>=) => {};
    (!=) => {};
    (<) => {};
    (>) => {};
    // everything else is not allowed
    ($other:tt) => {std::compile_error!("Comparison operator not allowed. The only allowed comparison operators are ==, !=, <=, >=, <, >");}
}

/// Compare all values in a set to a common right hand side and decide whether the comparison returns `true` for *any of the values* in the set.
///
/// # Lazy Evaluation
/// If we write `let cond = any_of!({a,b}<c)`, this is equivalent to the hand coded `let cond = (a<c) && (b<c)`. That means that the comparisons are
/// evaluated [lazily](https://doc.rust-lang.org/reference/expressions/operator-expr.html#lazy-boolean-operators) from left to right. Once
/// the truth value of the expression can be determined, the evaluation stops. That means that e.g. for an expression `any_of!({1,some_func()}<5)`,
/// the function `some_func()` is not invoked.
///
/// # Macro Syntax and Examples
/// The macro is called as `any_of!({/*list of expressions*/} operator rhs)`, where operator can be any of the binary comparison operators, i.e.
/// `==`, `!=`, `<=`, `<`, `>`, and `>=`. The list of expressions on the left hand side is comma separated without a trailing comma. The right hand side
/// is an expression as well. The list of expressions can have a variadic number of elements but must have at least one. It must always be enclosed in
/// curly braces. The expressions on the left hand side need not be of the same type, but the comparison with the right hand side must be valid. In particular,
/// the expressions need not be numeric.
///
/// ## Examples
/// The following examples show how to use the macro.
/// ```
/// # use fluent_comparisons_macros::any_of;
/// use rand::prelude::*;
///
/// let square = |val|val*val;
/// // the following assertion holds
/// assert!(any_of!({4+4+1,square(7*2),120_i32.pow(2)}>8));
///
/// let v = vec![1, 2,3];
/// let mut rng = rand::thread_rng();
/// // the following assertion holds
/// assert!(any_of!( {rng.gen::<usize>(),v.len(),2,1+1,"hello world".len()} == v.len()));
/// ```
#[macro_export]
macro_rules! any_of {
    //TODO: DOCUMENT THIS VARIANT WITH MAP (WITHOUT RHS)
    ( {$($lh_sides:expr),+}.satisfies($($func:tt)+) ) => {
        {
            //$crate::__check_operator!($operator);
            let map_func = $($func)+;
            $( map_func($lh_sides) )||+
        }
    };

    //TODO: DOCUMENT THIS VARIANT WITH MAP (WITH RHS)
    ( {$($lh_sides:expr),+}.map($($func:tt)+) $operator:tt $rhs:expr) => {
        {
            $crate::__check_operator!($operator);
            let map_func = $($func)+;
            $( map_func($lh_sides) $operator $rhs)||+
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
/// # Lazy Evaluation
/// If we write `let cond = all_of!({a,b}<c)`, this is equivalent to the hand coded `let cond = (a<c) && (b<c)`. That means that the comparisons are
/// evaluated [lazily](https://doc.rust-lang.org/reference/expressions/operator-expr.html#lazy-boolean-operators) from left to right. Once
/// the truth value of the expression can be determined, the evaluation stops. That means that e.g. for an expression `all_of!({1,some_func()}<5)`,
/// the function `some_func()` is not invoked.
///
/// # Macro Syntax and Examples
/// The macro is called as `all_of!({/*list of expressions*/} operator rhs)`, where operator can be any of the binary comparison operators, i.e.
/// `==`, `!=`, `<=`, `<`, `>`, and `>=`. The list of expressions on the left hand side is comma separated without a trailing comma. The right hand side
/// is an expression as well. The list of expressions can have a variadic number of elements but must have at least one. It must always be enclosed in
/// curly braces. The expressions on the left hand side need not be of the same type, but the comparison with the right hand side must be valid. In particular,
/// the expressions need not be numeric.
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
    ( {$($lh_sides:expr),+} $operator:tt $rhs:expr)=> {
        {
            $crate::__check_operator!($operator);
            $( ($lh_sides $operator $rhs) )&&+
        }
    };
}

/// Compare all values in a set to a common right hand side and decide whether the comparison returns `true` for *none of the values* in the set.
///
/// # Lazy Evaluation
/// If we write `let cond = none_of!({a,b}<c)`, this is equivalent to the hand coded `let cond = (a<c) && (b<c)`. That means that the comparisons are
/// evaluated [lazily](https://doc.rust-lang.org/reference/expressions/operator-expr.html#lazy-boolean-operators) from left to right. Once
/// the truth value of the expression can be determined, the evaluation stops. That means that e.g. for an expression `none_of!({1,some_func()}<5)`,
/// the function `some_func()` is not invoked.
///
/// # Macro Syntax and Examples
/// The macro is called as `none_of!({/*list of expressions*/} operator rhs)`, where operator can be any of the binary comparison operators, i.e.
/// `==`, `!=`, `<=`, `<`, `>`, and `>=`. The list of expressions on the left hand side is comma separated without a trailing comma. The right hand side
/// is an expression as well. The list of expressions can have a variadic number of elements but must have at least one. It must always be enclosed in
/// curly braces. The expressions on the left hand side need not be of the same type, but the comparison with the right hand side must be valid. In particular,
/// the expressions need not be numeric.
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
    ( {$($lh_sides:expr),+} $operator:tt $rhs:expr)=> {
        {
            $crate::__check_operator!($operator);
            $( !($lh_sides $operator $rhs) )&&+
        }
    };
}

// TODO FINISH THIS UP, TEST IT AND DOCUMENT IT
// TODO: make this as simple as the ones above w/o recursion
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
