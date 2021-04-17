// the only thing that the integration tests need to tests, which was not tested in the crate
// is that the macros really can be accessed from the outside.

use fluent_comparisons::all_of;
use fluent_comparisons::any_of;
use fluent_comparisons::none_of;
use fluent_comparisons::exactly_one_of;

use assert2::assert as assert2;

#[test]
fn test_accessibility() {
    assert2!( all_of!({1,2,3} <= 4)
        && none_of!({-1,0,-100}>0)
        && exactly_one_of!({1,2,3,4} >= 4)
        && any_of!({-1,2,3}==2));
}
