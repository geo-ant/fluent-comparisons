//! This crate contains the macros for the fluent-comparisons crate


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
