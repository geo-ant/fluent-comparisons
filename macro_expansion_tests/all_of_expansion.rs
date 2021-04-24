use fluent_comparisons::all_of;

struct Dummy {
    pub length : usize,
}

pub fn something() {
    let first = all_of!({1,2,3}<4);

    let v = Dummy {length : 2};
    let second = all_of!( {v.length,2_usize.pow(2),3*4+1} == v.length);

    let square = |x|x*x;
    let third = all_of!({4+4+1,square(7*2),120_i32.pow(2)}<=8);
}