// the only thing that the integration tests need to tests, which was not tested in the crate
// is that the macros really can be accessed from the outside and that they can be chained.

#![allow(clippy::assertions_on_constants)]

use fluent_comparisons::all_of;
use fluent_comparisons::any_of;
use fluent_comparisons::none_of;

#[test]
fn test_chaining() {
    assert!(
        all_of!({1,2,3} <= 4)
            && none_of!({-1,0,-100}>0)
            && any_of!({-1,2,3}==2)
    );
}
