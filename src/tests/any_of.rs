use assert2::assert as assert2;

use crate::any_of;

#[test]
fn test_any_of_simple() {
    let v = vec! {1, 2, 3};
    assert2!(any_of!( {1,2*v.len(),4} == 6) == true);
    assert2!(any_of!( {1,-1,2} == 4) == false);
}