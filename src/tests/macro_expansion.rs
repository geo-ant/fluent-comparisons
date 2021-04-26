//! Approval tests for macro expansion using the [macrotest](https://crates.io/crates/macrotest) crate.

#[test]
/// test that the macro expansion is the same as a manually approved expansion test file
fn any_of_macro_expansion_is_same_as_approved_expansion() {
    // to update: delete the .expansion.rs file
    // and replace macrotest::expand_without_refresh with macrotest::expand
    macrotest::expand("macro_expansion_tests/any_of_expansion.rs");
}

#[test]
/// test that the macro expansion is the same as a manually approved expansion test file
fn any_of_macro_with_map_expansion_is_same_as_approved_expansion() {
    // to update: delete the .expansion.rs file
    // and replace macrotest::expand_without_refresh with macrotest::expand
    macrotest::expand("macro_expansion_tests/any_of_with_map_or_satisfies_expansion.rs");
}

#[test]
/// test that the macro expansion is the same as a manually approved expansion test file
fn all_of_macro_expansion_is_same_as_approved_expansion() {
    // to update: delete the .expansion.rs file
    // and replace macrotest::expand_without_refresh with macrotest::expand
    macrotest::expand("macro_expansion_tests/all_of_expansion.rs");
}

#[test]
/// test that the macro expansion is the same as a manually approved expansion test file
fn all_of_macro_with_map_expansion_is_same_as_approved_expansion() {
    // to update: delete the .expansion.rs file
    // and replace macrotest::expand_without_refresh with macrotest::expand
    macrotest::expand("macro_expansion_tests/all_of_with_map_or_satisfies_expansion.rs");
}

#[test]
/// test that the macro expansion is the same as a manually approved expansion test file
fn none_of_macro_expansion_is_same_as_approved_expansion() {
    // to update: delete the .expansion.rs file
    // and replace macrotest::expand_without_refresh with macrotest::expand
    macrotest::expand("macro_expansion_tests/none_of_expansion.rs");
}

#[test]
/// test that the macro expansion is the same as a manually approved expansion test file
fn none_of_macro_with_map_expansion_is_same_as_approved_expansion() {
    // to update: delete the .expansion.rs file
    // and replace macrotest::expand_without_refresh with macrotest::expand
    macrotest::expand("macro_expansion_tests/none_of_with_map_or_satisfies_expansion.rs");
}
