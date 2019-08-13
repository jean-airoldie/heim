//! Measurement units used in API.

pub use uom::si::f64::Time;
pub use uom::si::time;

/// Information measurement unit.
///
/// Base unit is `byte`.
#[derive(heim_derive::Unit, Copy, Clone, Debug, Default, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Information(u64);

impl Information {
    #[doc(hidden)]
    pub fn from_kilobytes(value: u64) -> Self {
        Self::new(value * 1_024)
    }
}

/// Ratio measurement unit.
///
/// It is dimensionless and represents the value in the `[0.0 … 1.0]` range
#[derive(heim_derive::Unit, Copy, Clone, Debug, Default, PartialOrd, PartialEq)]
pub struct Ratio(f32);
