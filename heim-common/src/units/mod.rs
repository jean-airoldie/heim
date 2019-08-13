//! Measurement units used in API.
//!
//! Check out the [`uom`](https://docs.rs/uom/) crate docs of how to use them.

pub use uom::si::f64::Time;
pub use uom::si::u64::Information;
pub use uom::si::{information, information_rate, time};

/// Ratio measurement unit.
///
/// It is dimensionless and represents the value in the `[0.0 … 1.0]` range
#[derive(heim_derive::Unit, Copy, Clone, Debug, Default, PartialOrd, PartialEq)]
pub struct Ratio(f32);
