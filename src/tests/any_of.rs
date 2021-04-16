use assert2::assert as assert2;

use crate::any_of;

#[test]
fn test_any_of_simple_literals() {

    let cond1 = any_of!( {1,2,4} == 6);
    let cond2 = any_of!( {1,4,4} == 4);
    assert2!(cond1 == false);
    assert2!(cond2 == true);
}