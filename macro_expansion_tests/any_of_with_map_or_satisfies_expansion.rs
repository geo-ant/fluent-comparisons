use fluent_comparisons::any_of;
#[derive(Copy,Clone)]
struct Dummy {
    pub length : usize,
}

pub fn something() {
    let cond1 = any_of!({1,2,3}.map(|x|x*x)<4);
    let is_even = |x|x%2==0;
    let cond2 = any_of!({1,2,3}.satisfy(is_even));


    let d1 = Dummy {length : 2};
    let d2 = Dummy {length : 3};
    let cond3 = any_of!( {d1,d2}.map(|d|d.length*2) == d1.length);
    let cond4 = any_of!( {d1,d2}.satisfy(|d|d.length==2));
}