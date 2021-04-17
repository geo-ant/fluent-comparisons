use crate::none_of;

#[test]
fn test_none_of_simple() {
    let v = vec![1, 2, 3];
    let cond1 = none_of!( {1,6*v.len(),4} == 6);
    let cond2 = none_of!( {1,-1,4} == 4);
    assert_eq!(cond1, true);
    assert_eq!(cond2, false);
    assert_eq!(none_of!( {2_i32.pow(2)+2,2*v.len(),4+2} == 6), false);
    assert_eq!(none_of!( {2_i32.pow(2)+2,2*v.len(),1234} == 6), false);
}
