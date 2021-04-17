#![allow(clippy::int_plus_one)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::assertions_on_constants)]

use crate::all_of;
use std::cell::Cell;

use rand::prelude::*;

// helper function to double a value
fn twice(val: usize) -> usize {
    2 * val
}

#[test]
// handcraft test cases to thest the logic of all_of for the case of operator ==
fn all_of_comparisons_give_correct_result_for_operator_equal() {
    // simple expressions of one or two literals
    assert!(all_of!({ 4 } == 4));
    assert_eq!(all_of!({ 2 } == 4), false);
    assert_eq!(all_of!( {4,2} == 4), false);
    assert_eq!(all_of!( {2,4} == 4), false);
    assert_eq!(all_of!( {2,2} == 2), true);
    assert_eq!(all_of!( {4,4} == 4), true);

    // expressions involving function calls and variable
    // make sure the non-matching expression changes position
    let v = vec![1, 2, 3];
    let two = 2;
    assert_eq!(all_of!( {6,2*v.len(),twice(3)} == twice(2)+two), true);
    assert_eq!(all_of!( {6+2,2*v.len(),twice(3)} == 3+3), false);
    assert_eq!(all_of!( {6,200*v.len(),twice(3)} == 4+2), false);
    assert_eq!(all_of!( {6,2*v.len(),twice(3)/2} == 5+1), false);

    // some other expressions for good measure
    assert_eq!(all_of!( {4,4,4,1,-1,2,4} == 4), false);
    assert_eq!(all_of!( {2_i32.pow(2)+2,2*v.len(),4+2,666-660} == 6), true);
    assert_eq!(
        all_of!( {6,6,6,6,6,2_i32.pow(2)+2,2*v.len(),1234} == 6),
        false
    );
}

#[test]
// due to the implementation of the macro it should not matter which comparison operator
// we actually put in there, but we'll test it anyway just to be sure
fn all_of_comparisons_give_correct_result_for_other_operators() {
    let v = vec!["hello", "there", "this", "is", "a", "test"];
    // <=
    assert_eq!(all_of!( {1,v.len(),twice(2)} <= 6), true);
    assert_eq!(all_of!( {2*2+1,v.len(),twice(2),-10,7} <= 6), false);
    // >=
    assert_eq!(all_of!({4,9,5,3,-1,2}>=-2), true);
    assert_eq!(all_of!({4,2*2*2,5,3,-1,2} >= 4), false);
    // <
    assert_eq!(all_of!( {1,v.len()-1,twice(2)} < 6), true);
    assert_eq!(all_of!( {2*2+1,v.len(),twice(2),-10,7} < 6), false);
    // >
    assert_eq!(all_of!({104,99,15,13,11,twice(4)} > v.len()), true);
    assert_eq!(all_of!({104,99,15,13,11,twice(4)} > v.len()*6), false);
}

#[test]
// use some randomness for asserting theories that should always be true. So I just calculate
// the expected result of the all of expression and then compare it to a known result that I
// calculate using standard library iterators.
// Inspired by the "Beautiful Testing" chapter in the book "Beautiful Code", O'Reilly
// https://www.oreilly.com/library/view/beautiful-code/9780596510046/
fn test_random_collection_of_values_behave_correctly() {
    let mut rng = thread_rng();

    for _ in 1..1000000 {
        let a = rng.gen_range(-5..5);
        let b = rng.gen_range(-5..5);
        let c = rng.gen_range(-5..5);
        let d = rng.gen_range(-5..5);
        let e = rng.gen_range(-5..5);
        let rhs = rng.gen_range(-5..5);

        assert_eq!(
            all_of!({a,b,c,d,e}==rhs),
            [a, b, c, d, e].iter().all(|v| v == &rhs)
        );
        assert_eq!(
            all_of!({a,b,c,d,e}<=rhs),
            [a, b, c, d, e].iter().all(|v| v <= &rhs)
        );
        assert_eq!(
            all_of!({a,b,c,d,e}>=rhs),
            [a, b, c, d, e].iter().all(|v| v >= &rhs)
        );
        assert_eq!(
            all_of!({a,b,c,d,e}>rhs),
            [a, b, c, d, e].iter().all(|v| v > &rhs)
        );
        assert_eq!(
            all_of!({a,b,c,d,e}<rhs),
            [a, b, c, d, e].iter().all(|v| v < &rhs)
        );
    }
}

#[test]
fn expressions_are_short_circuited_and_evaluated_left_to_right() {
    // use this as a variable that indicates mutable state
    // and helps me count how often the twice function was invoked
    let counter: Cell<i32> = Cell::new(0);
    let twice = |i: i32| {
        counter.set(counter.get() + 1);
        2 * i
    };

    let eval = all_of!({1,2,3,4,5,twice(3)}==1);
    assert!(!eval);
    assert_eq!(counter.get(), 0);

    let eval = all_of!({1,twice(3),twice(3),3}==1);
    assert!(!eval);
    assert_eq!(counter.get(), 1);
    counter.set(0);

    // this proves that the array way of doing things requires an eager evaluation
    let eval = [1, twice(3), twice(3), 3].iter().all(|val| val == &1);
    assert!(!eval);
    assert_eq!(counter.get(), 2);
}
