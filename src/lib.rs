#[cfg(test)]
mod tests;

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

///TODO DOCUMENT
#[macro_export]
macro_rules! any_of {
    //TODO CAUTION: THIS COULD BE CALLED WITH ONE ARGUMENT. MAKE SURE THAT THIS PRODUCES A VALID RESULT
    // expression like any_of!( {1,v.len(),4} < 3)
    ( {$($lh_sides:expr),+} $operator:tt $rhs:expr)=> {
        {
            any_of!(@intern lhs={$($lh_sides),+}, op=[$operator], rhs = $rhs, expanded = {false})
        }
    };

    // internal rules, recursion final case
    (@intern lhs = {$head:expr},op = [$op:tt], rhs = $rhs:expr, expanded = {$expression:expr}) => {
        $expression || $head==$rhs
    };

    // internal rules, recursion case
    (@intern lhs = {$head:expr, $($tail:expr),*}, op = [$op:tt], rhs = $rhs:expr, expanded = {$expression:expr}) =>{
        any_of!(@intern lhs={$($tail),*}, op=[$op], rhs = $rhs, expanded = {$expression || $head == $rhs})
    };

}
