use fluent_comparisons::all_of;
struct Dummy {
    pub length: usize,
}
pub fn something() {
    let first = { (1 < 4) && (2 < 4) && (3 < 4) };
    let v = Dummy { length: 2 };
    let second = {
        (v.length == v.length) && (2_usize.pow(2) == v.length) && (3 * 4 + 1 == v.length)
    };
    let square = |x| x * x;
    let third = { (4 + 4 + 1 <= 8) && (square(7 * 2) <= 8) && (120_i32.pow(2) <= 8) };
}
