use crate::none_of;
use rand::prelude::*;
use std::cell::Cell;

// helper function to double a value
fn twice(val: usize) -> usize {
    2 * val
}

#[test]
fn none_of_comparisons_give_correct_result_for_operator_equal() {
    // test simple expressions
    assert_eq!(none_of!({ 4 } == 2), true);
    assert_eq!(none_of!({ 4 } == 4), false);
    assert_eq!(none_of!({"four","two"} == "three"), true);
    assert_eq!(none_of!({"four","two"} == "two"), false);
    assert_eq!(none_of!({"two","two"} == "two"), false);
    assert_eq!(none_of!({"two","four"} == "two"), false);

    // more complex expressions
    let v = vec![1, 2, 3];
    let two = 2;
    assert_eq!(none_of!( {6,2*v.len(),twice(3)} == twice(2)+two), false);
    assert_eq!(none_of!( {6+2,2*v.len(),twice(3)} == 3+3), false);
    assert_eq!(none_of!( {6*6,200*v.len(),twice(3)} == 4+2), false);
    assert_eq!(none_of!( {6+1,2*v.len()+2,twice(3)/2} == 5+1), true);

    // more complex expr
    assert_eq!(
        none_of!( {2_i32.pow(2)+2,2*v.len(),4+2,666-660} == 6),
        false
    );
    assert_eq!(none_of!( {4,4,4,1,-1,2,4} == 3), true);
    assert_eq!(
        none_of!( {6,6,6,6,6,2_i32.pow(2)+2,2*v.len(),1234} == 6),
        false
    );
}

#[test]
fn test_none_of_comparisons_for_other_operators() {
    let v = vec!["hello", "there", "this", "is", "a", "test"];

    // !=
    assert_eq!(none_of!({6,2_usize.pow(2),v.len(),7}!=6), false);
    assert_eq!(none_of!({6,2_usize.pow(2)+2,v.len(),7-1}!=6), true);
    // <=
    assert_eq!(none_of!({7,8,9}<=6), true);
    assert_eq!(none_of!({6,7,8,9}<=6), false);
    // >=
    assert_eq!(none_of!({7,8,9}>=6), false);
    assert_eq!(none_of!({4,4,3,1}>=6), true);
    // >
    assert_eq!(none_of!({6,1,2,7,8,9}>6), false);
    assert_eq!(none_of!({4,4,3,1,5}>6), true);
    // <
    assert_eq!(none_of!({6,7,8,9}<6), true);
    assert_eq!(none_of!({7,8,6,2,7}<6), false);
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

    let eval = none_of!({1,2,3,4,5,twice(3)}==5);
    assert!(!eval);
    assert_eq!(counter.get(), 0);

    let eval = none_of!({1,twice(3),twice(3),3}==6);
    assert!(!eval);
    assert_eq!(counter.get(), 1);
    counter.set(0);

    // this proves that the array way of doing things requires an eager evaluation
    let eval = [1, twice(3), twice(3), 3].iter().all(|val| (val != &6));
    assert!(!eval);
    assert_eq!(counter.get(), 2);
}

#[test]
// use some randomness for asserting theories that should always be true. So I just calculate
// the expected result of the all of expression and then compare it to a known result that I
// calculate using standard library iterators.
// Inspired by the "Beautiful Testing" chapter in the book "Beautiful Code", O'Reilly
// https://www.oreilly.com/library/view/beautiful-code/9780596510046/
#[allow(clippy::nonminimal_bool)]
fn test_random_collection_of_values_behave_correctly() {
    let mut rng = thread_rng();

    for _ in 1..1000000 {
        let a = rng.gen_range(-5..5);
        let b = rng.gen_range(-5..5);
        let c = rng.gen_range(-5..5);
        let d = rng.gen_range(-5..5);
        let rhs = rng.gen_range(-5..5);

        assert_eq!(
            none_of!({a,b,c,d}==rhs),
            [a, b, c, d].iter().all(|v| !(v == &rhs))
        );
        assert_eq!(
            none_of!({a,b,c,d}<=rhs),
            [a, b, c, d].iter().all(|v| !(v <= &rhs))
        );
        assert_eq!(
            none_of!({a,b,c,d}>=rhs),
            [a, b, c, d].iter().all(|v| !(v >= &rhs))
        );
        assert_eq!(
            none_of!({a,b,c,d}>rhs),
            [a, b, c, d].iter().all(|v| !(v > &rhs))
        );
        assert_eq!(
            none_of!({a,b,c,d}<rhs),
            [a, b, c, d].iter().all(|v| !(v < &rhs))
        );
        assert_eq!(
            none_of!({a,b,c,d}!=rhs),
            [a, b, c, d].iter().all(|v| !(v != &rhs))
        );
    }
}
