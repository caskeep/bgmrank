extern crate enum_set;
#[cfg(test)]
extern crate float_cmp;
#[macro_use]
extern crate html5ever_atoms;
extern crate kuchiki;
extern crate selectors;

mod classifier;
mod data;
mod fetch;
mod helpers;
mod parser;
mod stats;

pub use data::*;
pub use fetch::*;
pub use stats::*;
