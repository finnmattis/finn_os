use num_traits::{NumAssignOps, NumCast, Signed};

mod bresenham;
mod octant;

pub(crate) use bresenham::Bresenham;
use octant::Octant;

/// A point in 2D space.
pub type Point<T> = (T, T);

pub(crate) trait SignedNum: Signed + Ord + Copy + NumCast + NumAssignOps {
    fn cast<T: NumCast>(value: T) -> Self {
        NumCast::from(value).unwrap()
    }
}

impl<T: Signed + Ord + Copy + NumCast + NumAssignOps> SignedNum for T {}
