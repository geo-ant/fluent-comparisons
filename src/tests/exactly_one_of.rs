use crate::exactly_one_of;

use assert2::assert as assert2;

#[test]
fn test_exactly_one_of_simple() {
    let v = vec!{1,2,3};
    let cond1 = exactly_one_of!( {1,2*v.len(),4} == 6);
    let cond2 = exactly_one_of!( {1,-1,2} == 4);

    assert2!(cond1 == true);
    assert2!(cond2 == false);
    assert2!(exactly_one_of!( {2_i32.pow(2)+2,2*v.len(),4+2} == 6)==false);
    assert2!(exactly_one_of!( {2_i32.pow(2)+2,2*v.len(),1234} == 6)==false);
    assert2!(exactly_one_of!( {2_i32.pow(2)+2,5*v.len(),1234} == 6)==true);
    assert2!(exactly_one_of!( {6} == 6)==true);
    assert2!(exactly_one_of!( {7} == 6)==false);
}