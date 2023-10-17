use crate::none_of;
use rand::prelude::*;

// helper function to double a value
fn twice(val: usize) -> usize {
    2 * val
}

#[test]
fn none_of_comparisons_give_correct_result_for_operator_equal() {
    // test simple expressions
    assert!(none_of!({ 2 }.map(|x: i32| -x) == 2));
    assert_eq!(none_of!({ 2 }.map(|x: i32| 2 * x) == 4), false);
    assert_eq!(none_of!({3,2,1}.map(|x:i32|x-2)<0), false);
    assert!(none_of!({3,2,1}.satisfy(|x|x%4==0)));

    // more complex expressions
    let v = [1, 2, 3];
    let two = 2;
    assert_eq!(
        none_of!( {1,2*v.len()-1,twice(4)}.map(|x|x+1) == twice(2)+two),
        false
    );
    assert_eq!(none_of!( {6+2,2*v.len(),twice(3)}.satisfy(|x|x==6)), false);

    // more complex expr
    assert_eq!(
        none_of!( {2_i32.pow(2)+2,2*v.len() as i32,4+2,666}.map(|x|x-660) == 6),
        false
    );
    assert!(none_of!( {4,4,4,1,3,2,4}.map(|x|2*x) == 3));
}

#[test]
fn test_none_of_comparisons_for_other_operators() {
    let v = ["hello", "there", "this", "is", "a", "test"];
    let twice = |x| x * 2;
    // !=
    assert_eq!(none_of!({3,2_usize.pow(2),v.len(),7}.map(twice)!=6), false);
    assert!(
        none_of!({6,2_usize.pow(2)+2,v.len(),7-1}.satisfy(|x|x!=6))
    );
    // <=
    assert!(none_of!({7,8,9}.satisfy(|x|x<=6)));
    assert_eq!(none_of!({7,7,8,9}.map(|x|x-1)<=6), false);
    // >=
    assert_eq!(none_of!({6,6,6}.map(|x|2*x+3)>=6), false);
    assert!(none_of!({4,4,3,1}.satisfy(|x|x>=6)));
    // >
    assert_eq!(none_of!({6,1,2,7,8,9}.satisfy(|x|x>6)), false);
    assert!(none_of!({4,4,3,1,5}.satisfy(|c|c>6)));
    // <
    assert!(none_of!({5,7,8,9}.map(|x|2*x)<6));
    assert_eq!(none_of!({7,8,6,2,7}.satisfy(|x|x<6)), false);
}

#[test]
// use some randomness for asserting theories that should always be true. So I just calculate
// the expected result of the all of expression and then compare it to a known result that I
// calculate using standard library iterators.
// Inspired by the "Beautiful Testing" chapter in the book "Beautiful Code", O'Reilly
// https://www.oreilly.com/library/view/beautiful-code/9780596510046/
#[allow(clippy::nonminimal_bool)]
#[allow(clippy::int_plus_one)]
fn test_random_collection_of_values_behave_correctly() {
    let mut rng = thread_rng();

    for _ in 1..100000 {
        let a = rng.gen_range(-5..5);
        let b = rng.gen_range(-5..5);
        let c = rng.gen_range(-5..5);
        let d = rng.gen_range(-5..5);
        let rhs = rng.gen_range(-5..5);

        assert_all_eq!(
            none_of!({a,b,c,d}.map(|x|x+1)==rhs+1),
            none_of!({a,b,c,d}.satisfy(|x|x+1==rhs+1)),
            [a, b, c, d].iter().map(|x| x + 1).all(|v| !(v == rhs + 1))
        );
        assert_all_eq!(
            none_of!({a,b,c,d}.map(|x|2*x)<=2*rhs),
            none_of!({a,b,c,d}.satisfy(|x|2*x <= 2*rhs)),
            [a, b, c, d].iter().map(|x| 2 * x).all(|v| !(v <= 2 * rhs))
        );
        assert_all_eq!(
            none_of!({a,b,c,d}.map(|x|x+1)>=rhs+1),
            none_of!({a,b,c,d}.satisfy(|x|x+1>=rhs+1)),
            [a, b, c, d].iter().map(|x| x + 1).all(|v| !(v >= rhs + 1))
        );
        assert_all_eq!(
            none_of!({a,b,c,d}.map(|x|333*x)>333*rhs),
            none_of!({a,b,c,d}.satisfy(|x|333*x>333*rhs)),
            [a, b, c, d]
                .iter()
                .map(|x| 333 * x)
                .all(|v| !(v > rhs * 333))
        );
        assert_all_eq!(
            none_of!({a,b,c,d}.map(|x|x-2)<rhs-2),
            none_of!({a,b,c,d}.satisfy(|x|x-2<rhs-2)),
            [a, b, c, d].iter().map(|x| x - 2).all(|v| !(v < rhs - 2))
        );
        assert_all_eq!(
            none_of!({a,b,c,d}.map(|x|x+1)!=rhs),
            none_of!({a,b,c,d}.satisfy(|x|x+1!=rhs)),
            [a, b, c, d].iter().map(|x| x + 1).all(|v| !(v != rhs))
        );
    }
}
