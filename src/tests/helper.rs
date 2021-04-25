/// A helper macro to assert that two or more expressions are the same.
/// This is just easier than writing multiple assert_eq! to the same effect.
/// But it has the drawback of not being able to pass additional error messages
///
/// ```
/// assert_all_eq!(expression1, expression2, expression3,...)
/// ``
/// CAUTION: this does not have an additional info text
macro_rules! assert_all_eq {
    ($lhs:expr, $($expressions:expr),+) => {
        $(
            ::std::assert_eq!($lhs,$expressions);
        )+
    };


}

#[test]
fn test_assert_all_eq_success_cases() {
    assert_all_eq!(5,4+1,3+2);
    assert_all_eq!(5usize.pow(2),25,6*6-11,20+5);
    assert_all_eq!(5*5,4*4+9);
}

#[test]
#[should_panic]
fn test_assert_all_eq_failure_case1() {
    assert_all_eq!(5,4+1,3+3);
}

#[test]
#[should_panic]
fn test_assert_all_eq_failure_case2() {
    assert_all_eq!(5usize.pow(2),25,6*6-10,20+5);
}
