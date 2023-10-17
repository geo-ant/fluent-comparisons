#![allow(clippy::int_plus_one)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::assertions_on_constants)]

use crate::any_of;
use rand::prelude::*;
use std::cell::Cell;
use std::ops::Mul;

// a helper function that squares its argument
fn square<T: Mul<Output = T> + Copy>(val: T) -> T {
    val * val
}

#[test]
fn any_of_comparisons_give_correct_result_for_operator_equal() {
    // test very simple expressions with 2 elements
    assert!(any_of!({ 4 } == 4));
    assert_eq!(any_of!({ 2 } == 4), false);
    assert!(any_of!( {4,2} == 4));
    assert!(any_of!( {2,4} == 4));
    assert!(any_of!( {4,4} == 4));
    assert_eq!(any_of!( {4,2} == 1), false);
    assert_eq!(any_of!( {2,4} == 1), false);

    // test more complicated expressions with 3 elements
    //assert_eq!()

    let v = [1, 2];
    assert!(
        any_of!( {2f64.cos(),3f64.sin(),0f64.cos()} <= -std::f64::EPSILON)
    );
    assert!(any_of!( {v.len(),2_usize.pow(2),3*4+1} == v.len()));
    assert!(any_of!( {3_usize.pow(2),333,1+1} == v.len()));
    assert!(any_of!( {v.len(),2,1+1} == v.len()));
    assert!(any_of!( {v.len(),2,1} == v.len().pow(1)));
    assert_eq!(
        any_of!( {v.len()/2,v.len()-1,v.len().pow(4)} == v.len()),
        false
    );
}

#[test]
// the macro should not care about which comparison operator I plug in, but I want
// to make sure it works, nonetheless
fn test_any_of_comparisons_for_other_comparison_operators() {
    // !=
    assert!(any_of!({2,2,-2,123}!=2));
    assert_eq!(any_of!({square(2),2*2}!=4), false);
    // <=
    assert!(any_of!({square(3),8,120,1}<=8));
    assert_eq!(any_of!({4+4+1,square(7*2),120_i32.pow(2)}<=8), false);
    // >=
    assert!(any_of!({-11,3}>=-11));
    assert_eq!(any_of!({square(2+1),-4}>=10), false);
    // <
    assert!(any_of!({square(3),7,120,1}<8));
    assert_eq!(any_of!({4+4+1,square(7*2),120_i32.pow(2)}<8), false);
    // >
    assert!(any_of!({-11,3}>-10));
    assert_eq!(any_of!({-11,3}>4), false);
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

    let eval = any_of!({1,2,3,4,5,twice(3)}==5);
    assert!(eval);
    assert_eq!(counter.get(), 0);

    let eval = any_of!({1,twice(3),twice(3),3}==6);
    assert!(eval);
    assert_eq!(counter.get(), 1);
    counter.set(0);

    // this proves that the array way of doing things requires an eager evaluation
    let eval = [1, twice(3), twice(3), 3].iter().any(|val| val == &6);
    assert!(eval);
    assert_eq!(counter.get(), 2);
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

        assert_eq!(
            any_of!({a,b,c,d}==rhs),
            [a, b, c, d].iter().any(|v| v == &rhs)
        );
        assert_eq!(
            any_of!({a,b,c,d}<=rhs),
            [a, b, c, d].iter().any(|v| v <= &rhs)
        );
        assert_eq!(
            any_of!({a,b,c,d}>=rhs),
            [a, b, c, d].iter().any(|v| v >= &rhs)
        );
        assert_eq!(
            any_of!({a,b,c,d}>rhs),
            [a, b, c, d].iter().any(|v| v > &rhs)
        );
        assert_eq!(
            any_of!({a,b,c,d}<rhs),
            [a, b, c, d].iter().any(|v| v < &rhs)
        );
        assert_eq!(
            any_of!({a,b,c,d}!=rhs),
            [a, b, c, d].iter().any(|v| v != &rhs)
        );
    }
}
