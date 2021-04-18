//! This library provides a number of macros to make multicomparison expressions less annoying to
//! write while still keeping the benefits of hand-written code, such as lazy evaluation and
//! boolean short circuiting. Go to any of the macros to find out about their usage.

pub use fluent_comparisons_macros::any_of;

pub use fluent_comparisons_macros::all_of;

pub use fluent_comparisons_macros::none_of;

#[cfg(test)]
mod tests;
