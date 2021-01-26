//! Traits for Weierstrass elliptic curve points

use super::Curve;
use crate::FieldBytes;
use subtle::{Choice, CtOption};

/// Point compression settings
pub trait Compression {
    /// Should point compression be applied by default?
    const COMPRESS_POINTS: bool;
}

/// Point compaction settings
pub trait Compaction {
    /// Should point compaction be applied by default?
    const COMPACT_POINTS: bool;
}

/// Attempt to decompress an elliptic curve point from its x-coordinate and
/// a boolean flag indicating whether or not the y-coordinate is odd.
pub trait Decompress<C: Curve>: Sized {
    /// Attempt to decompress an elliptic curve point
    fn decompress(x: &FieldBytes<C>, y_is_odd: Choice) -> CtOption<Self>;
}

/// Attempt to compact an elliptic curve point from its x-coordinate and
pub trait Decompact<C: Curve>: Sized {
    /// Attempt to decompact an elliptic curve point
    fn decompact(x: &FieldBytes<C>) -> CtOption<Self>;
}
