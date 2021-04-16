use crate::all_of;
use assert2::assert as assert2;
use std::rc::Rc;
use std::cell::{RefCell, Cell};

#[test]
fn test_simple() {
    let v = vec! {1, 2, 3};
    let cond1 = all_of!( {1,2*v.len(),4} == 6);
    let cond2 = all_of!( {1,-1,2} == 4);
    assert2!(cond1 == false);
    assert2!(cond2 == false);
    assert2!(all_of!( {2_i32.pow(2)+2,2*v.len(),4+2} == 6)==true);
    assert2!(all_of!( {2_i32.pow(2)+2,2*v.len(),1234} == 6)==false);
    assert2!(all_of!( {6} == 6)==true);
    assert2!(all_of!( {7} == 6)==false);
}

#[test]
fn test_lazyness() {
    // use this as a variable that indicates mutable state
    // and helps me count how often the twice function was invoked
    let counter: Cell<i32> = Cell::new(0);
    let mut twice = |i: i32| {
        unsafe { counter.set(counter.get() + 1) };
        2 * i
    };

    let eval = all_of!({1,2,3,4,5,twice(3)}==1);
    assert2!(eval == false);
    assert2!(counter.get() == 0);

    let eval = all_of!({1,twice(3),twice(3),3}==1);
    assert2!(eval == false);
    assert2!(counter.get() == 1);
    counter.set(0);

    // this proves that the array way of doing things requires an eager evaluation
    let eval = [1,twice(3),twice(3),3].iter().all(|val|val==&1);
    assert2!(eval == false);
    assert2!(counter.get() == 2);
}