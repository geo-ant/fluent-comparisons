#![allow(clippy::int_plus_one)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::assertions_on_constants)]

use crate::any_of;
use rand::prelude::*;

#[test]
fn any_of_with_map_comparisons_give_correct_result_for_operator_equal() {
    // test very simple expressions
    assert_eq!(any_of!({3,4,5}.satisfy(|x|x%2==0)), true);
    assert_eq!(any_of!({3,4,5}.map(|x|x%2)==0), true);
    assert_eq!(any_of!({ 2 }.map(|x| x * x) == 4), true);
    assert_eq!(any_of!({ 2,4 }.map(|x:i32|x.pow(10)) == 4), false);

    // test more complicated expressions

    let v = vec![1, 2];
    assert_eq!(
        any_of!( {2f64.cos(),3f64.sin(),0f64.cos()} <= -std::f64::EPSILON),
        true
    );
    assert_eq!(
        any_of!( {v.len(),2_usize.pow(2),3*4+1}.map(|x:usize|-(x as i64)) == -(v.len() as i64)),
        true
    );
    assert_eq!(
        any_of!( {v.len()/2,v.len()-1,v.len().pow(4)}.satisfy(|x|x==v.len())),
        false
    );
}

#[test]
// the macro should not care about which comparison operator I plug in, but I want
// to make sure it works, nonetheless
fn test_any_of_comparisons_for_other_comparison_operators() {
    let square = |x| x * x;
    // !=
    assert_eq!(any_of!({2,2,2,2}.map(|x|x-1)!=2), true);
    assert_eq!(any_of!({2,1+1}.map(square)!=4), false);
    // <=
    assert_eq!(any_of!({square(3),8,120,1}.satisfy(|x|x<=8)), true);
    assert_eq!(
        any_of!({4+4+1,square(7*2),120_i32.pow(2)}.map(|x|x-1)<=8),
        true
    );
    // >=
    assert_eq!(any_of!({-11,3}.satisfy(|x|x>=2)), true);
    assert_eq!(any_of!({40,50}.map(|x|x/10)>=10), false);
    // <
    assert_eq!(any_of!({14,20,120,30}.map(|x|x/2)<8), true);
    assert_eq!(any_of!({4+1,square(2),2_i32.pow(2)}.map(|x|x+5)<8), false);
    // >
    assert_eq!(any_of!({-11,3}.satisfy(|x|x>-10)), true);
    assert_eq!(any_of!({-11,5}.map(|x|x-1)>4), false);
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
            any_of!({a,b,c,d}.map(|x|x*2) == rhs),
            any_of!({a,b,c,d}.satisfy(|x|x*2==rhs)),
            [a, b, c, d].iter().map(|x| x * 2).any(|v| v == rhs)
        );

        assert_all_eq!(
            any_of!({a,b,c,d}.map(|x|x+1)<=rhs),
            any_of!({a,b,c,d}.satisfy(|x|x+1<=rhs)),
            [a, b, c, d].iter().map(|x| x + 1).any(|v| v <= rhs)
        );

        assert_all_eq!(
            any_of!({a,b,c,d}.map(|x|x-1)>=rhs),
            any_of!({a,b,c,d}.satisfy(|x|x-1>=rhs)),
            [a, b, c, d].iter().map(|x| x - 1).any(|v| v >= rhs)
        );
        assert_all_eq!(
            any_of!({a,b,c,d}.map(|x|x+1)>rhs),
            any_of!({a,b,c,d}.satisfy(|x|x+1>rhs)),
            [a, b, c, d].iter().map(|x| x + 1).any(|v| v > rhs)
        );
        assert_all_eq!(
            any_of!({a,b,c,d}.map(|x|x+1)<rhs),
            any_of!({a,b,c,d}.satisfy(|x|x+1<rhs)),
            [a, b, c, d].iter().map(|x| x + 1).any(|v| v < rhs)
        );
        assert_all_eq!(
            any_of!({a,b,c,d}.map(|x|x+1)!=rhs+1),
            any_of!({a,b,c,d}.satisfy(|x|x+1!=rhs+1)),
            [a, b, c, d].iter().map(|x| x + 1).any(|v| v != rhs + 1)
        );
    }
}
