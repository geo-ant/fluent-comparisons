#[macro_export]
#[doc(hidden)]
/// # Internal Macro
/// This macro checks that the comparison operator in the expression is indeed allowed. If it is
/// allowed this macro evaluates to a unit/void statement. Otherwise it will give a compile error
/// explaining which operators are allowed
#[macro_export]
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

///TODO DOCUMENT ANY OF
#[macro_export]
macro_rules! any_of {
    //TODO CAUTION: THIS COULD BE CALLED WITH ONE ARGUMENT. MAKE SURE THAT THIS PRODUCES A VALID RESULT
    // expression like any_of!( {1,v.len(),4} < 3)
    ( {$($lh_sides:expr),+} $operator:tt $rhs:expr)=> {
        {
            $crate::__check_operator!($operator);
            any_of!(@internal lhs={$($lh_sides),+}, op=[$operator], rhs = $rhs, expanded = {false})
        }
    };
    // internal rules, recursion final case
    (@internal lhs = {$head:expr},op = [$op:tt], rhs = $rhs:expr, expanded = {$expanded:expr}) => {
        $expanded || $head $op $rhs
    };

    // internal rules, recursion case
    (@internal lhs = {$head:expr, $($tail:expr),*}, op = [$op:tt], rhs = $rhs:expr, expanded = {$expanded:expr}) =>{
        any_of!(@internal lhs={$($tail),*}, op=[$op], rhs = $rhs, expanded = {$expanded || $head $op $rhs})
    };
}

///TODO DOCUMENT
#[macro_export]
macro_rules! all_of {
    //TODO CAUTION: THIS COULD BE CALLED WITH ONE ARGUMENT. MAKE SURE THAT THIS PRODUCES A VALID RESULT
    // expression like any_of!( {1,v.len(),4} < 3)
    ( {$($lh_sides:expr),+} $operator:tt $rhs:expr)=> {
        {
            $crate::__check_operator!($operator);
            all_of!(@internal lhs={$($lh_sides),+}, op=[$operator], rhs = $rhs, expanded = {true})
        }
    };

    // internal rules, recursion final case
    (@internal lhs = {$head:expr},op = [$op:tt], rhs = $rhs:expr, expanded = {$expanded:expr}) => {
        $expanded && $head $op $rhs
    };

    // internal rules, recursion case
    (@internal lhs = {$head:expr, $($tail:expr),*}, op = [$op:tt], rhs = $rhs:expr, expanded = {$expanded:expr}) =>{
        all_of!(@internal lhs={$($tail),*}, op=[$op], rhs = $rhs, expanded = {$expanded && $head $op $rhs})
    };
}

///TODO DOCUMENT
#[macro_export]
macro_rules! none_of {
    //TODO CAUTION: THIS COULD BE CALLED WITH ONE ARGUMENT. MAKE SURE THAT THIS PRODUCES A VALID RESULT
    // expression like any_of!( {1,v.len(),4} < 3)
    ( {$($lh_sides:expr),+} $operator:tt $rhs:expr)=> {
        {
            $crate::__check_operator!($operator);
            none_of!(@internal lhs={$($lh_sides),+}, op=[$operator], rhs = $rhs, expanded = {true})
        }
    };

    // internal rules, recursion final case
    (@internal lhs = {$head:expr},op = [$op:tt], rhs = $rhs:expr, expanded = {$expanded:expr}) => {
        $expanded && !($head $op $rhs)
    };

    // internal rules, recursion case
    (@internal lhs = {$head:expr, $($tail:expr),*}, op = [$op:tt], rhs = $rhs:expr, expanded = {$expanded:expr}) =>{
        none_of!(@internal lhs={$($tail),*}, op=[$op], rhs = $rhs, expanded = {$expanded && !($head $op $rhs)})
    };
}

#[macro_export]
macro_rules! exactly_one_of {
    //TODO CAUTION: THIS COULD BE CALLED WITH ONE ARGUMENT. MAKE SURE THAT THIS PRODUCES A VALID RESULT
    // expression like any_of!( {1,v.len(),4} < 3)
    ( {$($lh_sides:expr),+} $operator:tt $rhs:expr)=> {
        {
            $crate::__check_operator!($operator);
            1u32 == exactly_one_of!(@internal lhs={$($lh_sides),+}, op=[$operator], rhs = $rhs, expanded = {0u32})
        }
    };

    // internal rules, recursion final case
    (@internal lhs = {$head:expr},op = [$op:tt], rhs = $rhs:expr, expanded = {$expanded:expr}) => {
        $expanded + if $head $op $rhs {1u32}else{0u32}
    };

    // internal rules, recursion case
    (@internal lhs = {$head:expr, $($tail:expr),*}, op = [$op:tt], rhs = $rhs:expr, expanded = {$expanded:expr}) =>{
        exactly_one_of!(@internal lhs={$($tail),*}, op=[$op], rhs = $rhs, expanded = {$expanded + if $head $op $rhs {1u32}else{0u32}})
    };
}
