//! test theorems that should always be true about the operators using random data

use crate::all_of;
use crate::any_of;
use crate::none_of;
use rand::prelude::*;

#[test]
fn test_random_collection_of_values_behave_correctly() {
    let mut rng = thread_rng();
    const EXPERIMENT_COUNT: usize = 1000000;

    let mut all_of_le = Vec::with_capacity(EXPERIMENT_COUNT);
    let mut none_of_geq = Vec::with_capacity(EXPERIMENT_COUNT);
    let mut any_of_le = Vec::with_capacity(EXPERIMENT_COUNT);

    for _ in 1..EXPERIMENT_COUNT {
        let a = rng.gen_range(-5..5);
        let b = rng.gen_range(-5..5);
        let c = rng.gen_range(-5..5);
        let rhs = rng.gen_range(-5..5);

        all_of_le.push(all_of!({a,b,c}<rhs));
        none_of_geq.push(none_of!({a,b,c}>=rhs));
        any_of_le.push(any_of!({a,b,c}<rhs));
    }
    // a sanity check
    // this is very very very sunlikely to fail because the tests are balanced so that the expected numbers
    // should be in the hundred-thousands. This is just a sanity check
    assert!(
        all_of_le.iter().filter(|v| **v).count() > 1,
        "Not all of these may be false"
    );
    assert!(
        none_of_geq.iter().filter(|v| **v).count() > 1,
        "Not all of these may be false"
    );
    assert!(
        any_of_le.iter().filter(|v| **v).count() > 1,
        "Not all of these may be false"
    );

    // now assert some general theorems that are always true about the comparisons in relation
    // to each other independent of the collection

    // 1) all_of!({...}<rhs) == none_of!({...}>=rhs)
    assert!(
        all_of_le
            .iter()
            .zip(none_of_geq.iter())
            .all(|(all_of_le, none_of_geq)| all_of_le == none_of_geq),
        "all_of and none_of give inconsistent results"
    );
    // 2) all_of!({...}<rhs) == true implies that any_of!({...}< rhs) == true
    assert!(
        all_of_le
            .iter()
            .zip(any_of_le.iter())
            .all(|(all_of_le, any_of_le)| if *all_of_le { *any_of_le } else { true }),
        "all_of and any_of give inconsistent results"
    );
    // 3) none_of!({...}>= rhs) == true implies that any_of!({...}<rhs) == true
    assert!(
        none_of_geq
            .iter()
            .zip(any_of_le.iter())
            .all(|(none_of_geq, any_of_le)| if *none_of_geq { *any_of_le } else { true }),
        "none_of and any_of give inconsistent results"
    );
}
