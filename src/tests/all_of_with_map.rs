#![allow(clippy::int_plus_one)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::assertions_on_constants)]

use crate::all_of;
use rand::prelude::*;

#[test]
fn any_of_with_map_comparisons_give_correct_result_for_operator_equal() {
    // test very simple expressions
    assert_eq!(all_of!({2,4,6}.satisfy(|x|x%2==0)), true);
    assert_eq!(all_of!({2,4,6}.map(|x|x%2)==0), true);
    assert_eq!(all_of!({ 2 }.map(|x| x * x) == 4), true);
    assert_eq!(all_of!({ 4,4 }.map(|x:i32|x.pow(2)) == 4), false);

    // test more complicated expressions

    let v = vec![1, 2];
    let twice = |x| 2 * x;
    let two = 2;
    assert_eq!(
        all_of!( {6,3*v.len(),twice(3)}.map(twice) == 2*(twice(2)+two)),
        true
    );
    assert_eq!(
        all_of!( {2,2_usize.pow(1),1+1}.map(|x:usize|-(x as i64)) == -(v.len() as i64)),
        true
    );
    assert_eq!(
        all_of!( {2,1+1,v.len()}.satisfy(|x|x==v.len().pow(1))),
        true
    );
}

#[test]
// due to the implementation of the macro it should not matter which comparison operator
// we actually put in there, but we'll test it anyway just to be sure
fn all_of_comparisons_give_correct_result_for_other_operators() {
    let twice = |x: usize| x + x;

    let v = vec!["hello", "there", "this", "is", "a", "test"];
    // !=
    assert_eq!(all_of!( {6,v.len(),2+2+2}.map(|x|x-1) != 6), true);
    assert_eq!(all_of!( {2*2+1,v.len(),twice(3)}.satisfy(|x|x != 6)), false);

    // <=
    assert_eq!(all_of!( {1,v.len(),twice(2)}.satisfy(|x|x <= 6)), true);
    assert_eq!(
        all_of!( {5,v.len()-1,twice(2)}.map(|x|x+6) <= v.len()),
        false
    );
    // >=
    assert_eq!(all_of!({4,9,5,3,-1,2}.satisfy(|x|x>=-2)), true);
    assert_eq!(all_of!({4,2*2*2,5,10,11,22}.map(|x|x/5) >= 4), false);
    // <
    assert_eq!(all_of!( {1,v.len()-1,twice(2)}.satisfy(|x|x < 6)), true);
    assert_eq!(
        all_of!( {2*2+1,v.len()-5,twice(2),11-10,5}.map(|x|10*x) < 6),
        false
    );
    // >
    assert_eq!(
        all_of!({104,99,15,13,11,twice(4)}.satisfy(|x|x > v.len())),
        true
    );
    assert_eq!(
        all_of!({104,99,36+15,36+13,36+11,36+twice(4)}.map(|x|x-6*6) > v.len()*6),
        false
    );
}

#[test]
// use some randomness for asserting theories that should always be true. So I just calculate
// the expected result of the all of expression and then compare it to a known result that I
// calculate using standard library iterators.
// Inspired by the "Beautiful Testing" chapter in the book "Beautiful Code", O'Reilly
// https://www.oreilly.com/library/view/beautiful-code/9780596510046/
fn test_random_collection_of_values_behave_correctly() {
    let mut rng = thread_rng();

    for _ in 1..100000 {
        let a = rng.gen_range(-5..5);
        let b = rng.gen_range(-5..5);
        let c = rng.gen_range(-5..5);
        let d = rng.gen_range(-5..5);
        let rhs = rng.gen_range(-5..5);

        // check that the output truly maps and is the same as the iter based solution
        // also check that the satisfy(...) with an equivalent predicate works
        assert_all_eq!(
            all_of!({a,b,c,d}.map(|x|x*2) == rhs*2),
            all_of!({a,b,c,d}.satisfy(|x|x*2==rhs*2)),
            [a, b, c, d].iter().map(|x| x * 2).all(|v| v == rhs * 2)
        );

        assert_all_eq!(
            all_of!({a,b,c,d}.map(|x|x+1)<=rhs),
            all_of!({a,b,c,d}.satisfy(|x|x+1<=rhs)),
            [a, b, c, d].iter().map(|x| x + 1).all(|v| v <= rhs)
        );

        assert_all_eq!(
            all_of!({a,b,c,d}.map(|x|x-1)>=rhs),
            all_of!({a,b,c,d}.satisfy(|x|x-1>=rhs)),
            [a, b, c, d].iter().map(|x| x - 1).all(|v| v >= rhs)
        );
        assert_all_eq!(
            all_of!({a,b,c,d}.map(|x|x+1)>rhs),
            all_of!({a,b,c,d}.satisfy(|x|x+1>rhs)),
            [a, b, c, d].iter().map(|x| x + 1).all(|v| v > rhs)
        );
        assert_all_eq!(
            all_of!({a,b,c,d}.map(|x|x+1)<rhs),
            all_of!({a,b,c,d}.satisfy(|x|x+1<rhs)),
            [a, b, c, d].iter().map(|x| x + 1).all(|v| v < rhs)
        );
        assert_all_eq!(
            all_of!({a,b,c,d}.map(|x|x+1)!=rhs+1),
            all_of!({a,b,c,d}.satisfy(|x|x+1!=rhs+1)),
            [a, b, c, d].iter().map(|x| x + 1).all(|v| v != rhs + 1)
        );
    }
}
