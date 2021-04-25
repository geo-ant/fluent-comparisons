#![allow(clippy::int_plus_one)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::assertions_on_constants)]

use crate::any_of;

#[test]
fn any_of_comparisons_give_correct_result_for_operator_equal() {
    // test very simple expressions with 2 elements
    assert_eq!(any_of!({3,4,5}.satisfy(|x|x%2==0)), true);
    assert_eq!(any_of!({3,4,5}.map(|x|x%2)==0), true);
    assert_eq!(any_of!({ 4 }.map(|x|x*x) == 4), true);
    assert_eq!(any_of!({ 2 } == 4), false);
    assert_eq!(any_of!( {4,2} == 4), true);
    assert_eq!(any_of!( {2,4} == 4), true);
    assert_eq!(any_of!( {4,4} == 4), true);
    assert_eq!(any_of!( {4,2} == 1), false);
    assert_eq!(any_of!( {2,4} == 1), false);

    // test more complicated expressions with 3 elements
    //assert_eq!()

    let v = vec![1, 2];
    assert_eq!(
        any_of!( {2f64.cos(),3f64.sin(),0f64.cos()} <= -std::f64::EPSILON),
        true
    );
    assert_eq!(any_of!( {v.len(),2_usize.pow(2),3*4+1} == v.len()), true);
    assert_eq!(any_of!( {3_usize.pow(2),333,1+1} == v.len()), true);
    assert_eq!(any_of!( {v.len(),2,1+1} == v.len()), true);
    assert_eq!(any_of!( {v.len(),2,1} == v.len().pow(1)), true);
    assert_eq!(
        any_of!( {v.len()/2,v.len()-1,v.len().pow(4)} == v.len()),
        false
    );
}
