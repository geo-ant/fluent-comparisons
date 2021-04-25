use fluent_comparisons::none_of;
struct Dummy {
    pub length: usize,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::marker::Copy for Dummy {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Dummy {
    #[inline]
    fn clone(&self) -> Dummy {
        {
            let _: ::core::clone::AssertParamIsClone<usize>;
            *self
        }
    }
}
pub fn something() {
    let cond1 = {
        let map_func = |x| x * x;
        !(map_func(1) < 4) && !(map_func(2) < 4) && !(map_func(3) < 4)
    };
    let is_even = |x| x % 2 == 0;
    let cond2 = {
        let map_func = is_even;
        !(map_func(1) == true) && !(map_func(2) == true) && !(map_func(3) == true)
    };
    let d1 = Dummy { length: 2 };
    let d2 = Dummy { length: 3 };
    let cond3 = {
        let map_func = |d| d.length * 2;
        !(map_func(d1) == d1.length) && !(map_func(d2) == d1.length)
    };
    let cond4 = {
        let map_func = |d| d.length == 2;
        !(map_func(d1) == true) && !(map_func(d2) == true)
    };
}
