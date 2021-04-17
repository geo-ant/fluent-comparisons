use crate::any_of;

#[test]
fn any_of_comparisons_give_correct_result_for_operator_equal() {

    // test very simple expressions with 2 elements
    assert_eq!(any_of!( {4} == 4), true);
    assert_eq!(any_of!( {2} == 4), false);
    assert_eq!(any_of!( {4,2} == 4), true);
    assert_eq!(any_of!( {2,4} == 4), true);
    assert_eq!(any_of!( {4,4} == 4), true);
    assert_eq!(any_of!( {4,2} == 1), false);
    assert_eq!(any_of!( {2,4} == 1), false);

    // test more complicated expressions with 3 elements
    //assert_eq!()


    //let v = vec! {1, 2, 3};

}