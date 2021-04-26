// the only thing that the integration tests need to tests, which was not tested in the crate
// is that the macros really can be accessed from the outside and that they can be chained.

#![allow(clippy::assertions_on_constants)]

use fluent_comparisons::all_of;
use fluent_comparisons::any_of;
use fluent_comparisons::none_of;

#[test]
// test chaining the expressions
fn test_chaining() {
    assert!(all_of!({1,2,3} <= 4) && none_of!({-1,0,-100}>0) && any_of!({-1,2,3}==2)
        && all_of!({2,4,6}.satisfy(|x|x%2==0)) && none_of!({2,3,4}.map(|x|x-1)>=4) && any_of!({1,2,3}.map(|x|x-2)<0));
}

#[test]
fn test_nesting() {
    assert!(all_of!( {
        all_of!({1,2,3} <= 4),
        none_of!({-1,0,-100}>0),
        any_of!({-1,2,3}==2),
        all_of!({2,4,6}.satisfy(|x|x%2==0)),
        none_of!({2,3,4}.map(|x|x-1)>=4),
        any_of!({1,2,3}.map(|x|x-2)<0)
    }.map(|b: bool| !b) == false));
}