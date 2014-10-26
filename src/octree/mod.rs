use std::num::Primitive;
use std::fmt::Show;
pub use self::volume::Volume;

mod volume;

static DEFAULT_CAPACITY: uint = 8;

/// A trait that must be implemented by types that are going to be
/// inserted into an `Octree`.
pub trait Index<T: Primitive + Show> {
    fn octree_index(&self) -> [T, ..3];
}

pub struct Octree<T: Primitive + Show, I: Index<T> + Clone> {
    /// Maximum number of items to store before subdivision.
    capacity: uint,
    /// Items in the node.
    items: Vec<I>,
    /// Bounding volume of the node.
    volume: Volume<T>,
    /// The octants of the node, in order of NW, NE, SW, SE, starting
    /// from the upper half.
    octants: Option<[Box<Octree<T, I>>, ..8]>
}

impl<T: Primitive + Show, I: Index<T> + Clone> Octree<T, I> {
    /// Constructs a new, empty `Octree` with bounding volume `vol`
    /// and default node capacity of `DEFAULT_CAPACITY`.
    #[inline]
    pub fn new(vol: Volume<T>) -> Octree<T, I> {
        Octree {
            capacity: DEFAULT_CAPACITY,
            items: Vec::with_capacity(DEFAULT_CAPACITY),
            volume: vol,
            octants: None
        }
    }

    /// Creates an empty `Octree` with volume `vol` and `capacity`.
    #[inline]
    pub fn with_capacity(vol: Volume<T>, capacity: uint) -> Octree<T, I> {
        Octree {
            capacity: capacity,
            items: Vec::with_capacity(capacity),
            volume: vol,
            octants: None
        }
    }

    /// Returns the number of items in the tree.
    #[inline]
    pub fn len(&self) -> uint {
        let mut len = self.items.len();
        match self.octants {
            Some(ref octants) => for ref node in octants.iter() {
                len += node.len();
            },
            None => {}
        }
        len
    }

    /// Inserts an `item` into the tree, subdividing it if necessary.
    #[inline]
    pub fn insert(&mut self, item: I) -> bool {
        // item must exist inside this quads' space.
        if !self.volume.contains(&item.octree_index()) {
            return false;
        }
        
        // Insert item it there's room.
        if self.items.len() < self.capacity {
            self.items.push(item.clone());
            return true;
        }
        
        match self.octants {
            Some(ref mut octants) => for node in octants.iter_mut() {
                if node.insert(item.clone()) {
                    return true;
                }
            },
            None => self.subdivide()
        }
        
        false
    }

    /// Returns all items inside the volume `vol`.
    #[inline]
    pub fn get_in_volume<'a>(&'a self, vol: &Volume<T>) -> Vec<&'a I> {
        let mut items = Vec::new();

        // Return empty vector if vol does not intersect.
        if !self.volume.intersects(vol) {
            return items;
        }

        // Add items for this node.
        for item in self.items.iter() {
            if vol.contains(&item.octree_index()) {
                items.push(item);
            }
        }
        
        match self.octants {
            Some(ref octants) => {
                for ref node in octants.iter() {
                    items.push_all(node.get_in_volume(vol).as_slice());
                }
                items
            },
            None => items
        }
    }

    /// Creates eight equal sized subtrees for this node.
    #[inline]
    fn subdivide(&mut self) {
        let cap = self.capacity;
        let min = self.volume.min;
        let max = self.volume.max;
        let (hw, hh, hd) = (half(max[0]), half(max[1]), half(max[2]));
        
        self.octants = Some([
            // upper
            box Octree::with_capacity(Volume::new([min[0], min[1], min[2]], [hw, hh, hd]), cap),
            box Octree::with_capacity(Volume::new([min[0] + hh, min[1], min[2]], [max[0], hh, hd]), cap),
            box Octree::with_capacity(Volume::new([min[0], min[1] + hh, min[2]], [hw, max[1], hd]), cap),
            box Octree::with_capacity(Volume::new([min[0] + hw, min[1] + hh, min[2]], [max[0], max[1], hd]), cap),
            // lower
            box Octree::with_capacity(Volume::new([min[0], min[1], hd], [hw, hh, max[2]]), cap),
            box Octree::with_capacity(Volume::new([min[0] + hh, min[1], hd], [max[0], hh, max[2]]), cap),
            box Octree::with_capacity(Volume::new([min[0], min[1] + hh, hd], [hw, max[1], max[2]]), cap),
            box Octree::with_capacity(Volume::new([min[0] + hw, min[1] + hh, hd], [max[0], max[1], max[2]]), cap)
        ]);
    }
}

#[inline]
fn half<T: Primitive + Show>(n: T) -> T {
    n.div(&NumCast::from(2u).unwrap())
}
