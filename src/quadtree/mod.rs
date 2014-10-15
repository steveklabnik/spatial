use std::num::Primitive;
use std::fmt::Show;
pub use self::volume::Volume;

mod volume;

static DEFAULT_CAPACITY: uint = 8;

/// A trait that must be implemented by types that are going to be
/// inserted into a `Quadtree`.
pub trait Index<T: Primitive + Show> {
    fn x(&self) -> T;
    fn y(&self) -> T;
}

pub struct Quadtree<T: Primitive + Show, P: Index<T> + Clone> {
    /// Maximum number of items to store before subdivision.
    capacity: uint,
    /// Items in this quadtree node.
    items: Vec<P>,
    /// Bounding volume of this node.
    pub volume: Volume<T>,

    /// The four quadrants of this node, in order of NW, NE, SW, SE.
    pub quadrants: Option<[Box<Quadtree<T, P>>, ..4]>
}

impl<T: Primitive + Show, P: Index<T> + Clone> Quadtree<T, P> {
    /// Constructs a new, empty `Quadtree` with bounding volume `vol`
    /// and default node capacity of `DEFAULT_CAPACITY`.
    #[inline]
    pub fn new(vol: Volume<T>) -> Quadtree<T, P> {
        Quadtree {
            capacity: DEFAULT_CAPACITY,
            items: Vec::with_capacity(DEFAULT_CAPACITY),
            volume: vol,
            quadrants: None
        }
    }

    /// Creates an empty quadtree with volume `vol` and `capacity`.
    #[inline]
    pub fn with_capacity(vol: Volume<T>, capacity: uint) -> Quadtree<T, P> {
        Quadtree {
            capacity: capacity,
            items: Vec::with_capacity(capacity),
            volume: vol,
            quadrants: None
        }
    }

    /// Returns the number of items in the tree.
    #[inline]
    pub fn len(&self) -> uint {
        let mut len = self.items.len();

        match self.quadrants {
            Some(ref quadrants) => for ref node in quadrants.iter() {
                len += node.len();
            },
            None => {}
        }

        len
    }

    /// Inserts an `item` into the quadtree, subdividing it if
    /// necessary.
    #[inline]
    pub fn insert(&mut self, item: P) -> bool {
        // item must exist inside this quads' space.
        if !self.volume.contains((item.x(), item.y())) {
            return false;
        }

        // Insert item it there's room.
        if self.items.len() < self.capacity {
            self.items.push(item.clone());
            return true;
        }

        match self.quadrants {
            Some(ref mut quadrants) => for node in quadrants.iter_mut() {
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
    pub fn get_in_volume<'a>(&'a self, vol: Volume<T>) -> Vec<&'a P> {
        let mut items = Vec::new();

        // Return empty vector if vol does not intersect.
        if !self.volume.intersects(vol) {
            return items;
        }

        // Add items for this node.
        for item in self.items.iter() {
            if vol.contains((item.x(), item.y())) {
                items.push(item);
            }
        }

        match self.quadrants {
            Some(ref quadrants) => {
                for ref node in quadrants.iter() {
                    items.push_all(node.get_in_volume(vol).as_slice());
                }
                items
            },
            None => items
        }
    }

    /// Creates four equal sized subtrees for this node.
    #[inline]
    fn subdivide(&mut self) {
        let (x, y) = self.volume.min;
        let (w, h) = self.volume.max;
        let (hw, hh) = (half(w), half(h));

        self.quadrants = Some([
            box Quadtree::with_capacity(Volume::new((x, y), (hw, hh)), self.capacity),
            box Quadtree::with_capacity(Volume::new((x + hh, y), (w, hh)), self.capacity),
            box Quadtree::with_capacity(Volume::new((x, y + hh), (hw, h)), self.capacity),
            box Quadtree::with_capacity(Volume::new((x + hw, y + hh), (w, h)), self.capacity)
        ]);
    }
}

#[inline]
fn half<T: Primitive + Show>(n: T) -> T {
    n.div(&NumCast::from(2u).unwrap())
}
