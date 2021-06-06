
use crate::exactly;

#[test]
fn test_exactly() {
    const FOUR : i32 = 2;
    assert_eq!(exactly!(FOUR of {1,2,2,4}.satisfy(|x|x==2)),true);
    assert_eq!(exactly!(1 of {1,2,2,4}.satisfy(|x|x==2)),false);
    assert!(exactly!(1 of {1,2}.map(|x|2*x) <= 2),true);
    assert!(exactly!(2 of {1,2}.map(|x|2*x) <= 2),false);
}