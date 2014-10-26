use std::fmt::Show;
use std::fmt;

/// A three-dimensional bounding volume for an `Octree` node.
pub struct Volume<T: Primitive> {
    /// The upper-top-left corner.
    pub min: [T, ..3],
    /// The lower-bottom-right corner.
    pub max: [T, ..3]
}

impl<T: Primitive> Volume<T> {
    /// Create a new bounding volume from three points, where both `min`
    /// and `max` are of format `[x, y, z]`.
    #[inline]
    pub fn new(min: [T, ..3], max: [T, ..3]) -> Volume<T> {
        Volume {
            min: min,
            max: max
        }
    }
    
    #[inline]
    pub fn min(&self) -> [T, ..3] {
        self.min
    }
    
    #[inline]
    pub fn max(&self) -> [T, ..3] {
        self.max
    }
    
    /// Returns `true` if `p` is inside the volume, `false` otherwise.
    #[inline]
    pub fn contains(&self, p: &[T, ..3]) -> bool {
        let min = self.min;
        let max = self.max;
        
        p[0] >= min[0] && p[0] <= max[0] &&
            p[1] >= min[1] && p[1] <= max[1] &&
            p[2] >= min[2] && p[2] <= max[2]
    }
    
    /// Returns `true` if `other` intersects the volume, `false`
    /// otherwise.
    #[inline]
    pub fn intersects(&self, other: &Volume<T>) -> bool {
        let min = self.min;
        let max = self.max;
        min[0] < other.max[0] && max[0] > other.min[0] &&
            min[1] < other.max[1] && max[1] > other.min[1] &&
            min[2] < other.max[2] && max[2] > other.min[2]
    }
}

impl<T: Primitive + Show> Show for Volume<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let min = self.min;
        let max = self.max;
        write!(f, "[[{} {} {}] [{} {} {}]]", 
               min[0], min[1], min[2], max[0], max[1], max[2])
    }
}
