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

pub struct Quadtree<'a, T: Primitive + Show, P: 'a + Index<T>> {
    /// Maximum number of items to store before subdivision.
    capacity: uint,
    /// Items in this quadtree node.
    items: Vec<&'a P>,
    /// Bounding volume of this node.
    pub volume: Volume<T>,

    pub nw: Option<Box<Quadtree<'a, T, P>>>,
    pub ne: Option<Box<Quadtree<'a, T, P>>>,
    pub sw: Option<Box<Quadtree<'a, T, P>>>,
    pub se: Option<Box<Quadtree<'a, T, P>>>
}

impl<'a, T: Primitive + Show, P: Index<T>> Quadtree<'a, T, P> {
    /// Creates an empty quadtree with volume `vol` and default
    /// capacity.
    #[inline]
    pub fn new(vol: Volume<T>) -> Quadtree<'a, T, P> {
        Quadtree {
            capacity: DEFAULT_CAPACITY,
            items: Vec::with_capacity(DEFAULT_CAPACITY),
            volume: vol,
            nw: None,
            ne: None,
            sw: None,
            se: None
        }
    }

    /// Creates an empty quadtree with volume `vol` and `capacity`.
    #[inline]
    pub fn with_capacity(vol: Volume<T>, capacity: uint) -> Quadtree<'a, T, P> {
        Quadtree {
            capacity: capacity,
            items: Vec::with_capacity(capacity),
            volume: vol,
            nw: None,
            ne: None,
            sw: None,
            se: None
        }
    }

    /// Returns the number of items in the tree.
    #[inline]
    pub fn len(&self) -> uint {
        let mut len = self.items.len();

        match self.nw {
            Some(ref node) => len += node.len(),
            None => {}
        }

        match self.ne {
            Some(ref node) => len += node.len(),
            None => {}
        }
        
        match self.sw {
            Some(ref node) => len += node.len(),
            None => {}
        }
        
        match self.se {
            Some(ref node) => len += node.len(),
            None => {}
        }

        len
    }

    /// Inserts an `item` into the quadtree, subdividing it if
    /// necessary.
    #[inline]
    pub fn insert(&mut self, item: &'a P) -> bool {
        // item must exist inside this quads' space.
        if !self.volume.contains((item.x(), item.y())) {
            return false;
        }

        // Insert item it there's room.
        if self.items.len() < self.capacity {
            self.items.push(item);
            return true;
        }

        match self.nw {
            None => self.subdivide(),
            _ => {}
        }

        match self.nw {
            Some(ref mut node) => if node.insert(item) {
                return true;
            },
            None => {}
        }

        match self.ne {
            Some(ref mut node) => if node.insert(item) {
                return true;
            },
            None => {}
        }

        match self.sw {
            Some(ref mut node) => if node.insert(item) {
                return true;
            },
            None => {}
        }

        match self.se {
            Some(ref mut node) => if node.insert(item) {
                return true;
            },
            None => {}
        }

        false
    }

    /// Returns all items inside the volume `vol`.
    #[inline]
    pub fn get_in_volume(&self, vol: Volume<T>) -> Vec<&P> {
        let mut items = Vec::new();

        // Return empty vector if vol does not intersect.
        if !self.volume.intersects(vol) {
            return items;
        }

        // Add items for this node.
        for item in self.items.iter() {
            if vol.contains((item.x(), item.y())) {
                items.push(*item);
            }
        }

        // Terminate search if there are no child nodes.
        match self.nw {
            None => return items,
            _ => ()
        }

        match self.nw {
            Some(ref node) => items.push_all(node.get_in_volume(vol)[]),
            None => {}
        }

        match self.ne {
            Some(ref node) => items.push_all(node.get_in_volume(vol)[]),
            None => {}
        }

        match self.sw {
            Some(ref node) => items.push_all(node.get_in_volume(vol)[]),
            None => {}
        }

        match self.se {
            Some(ref node) => items.push_all(node.get_in_volume(vol)[]),
            None => {}
        }

        items
    }

    /// Creates four equal sized subtrees for this node.
    #[inline]
    fn subdivide(&mut self) {
        let (x, y) = self.volume.min;
        let (w, h) = self.volume.max;
        let (hw, hh) = (half(w), half(h));
        
        self.nw = Some(box Quadtree::with_capacity(
            Volume::new((x, y), (hw, hh)), self.capacity));
        self.ne = Some(box Quadtree::with_capacity(
            Volume::new((x + hh, y), (w, hh)), self.capacity));
        self.sw = Some(box Quadtree::with_capacity(
            Volume::new((x, y + hh), (hw, h)), self.capacity));
        self.se = Some(box Quadtree::with_capacity(
            Volume::new((x + hw, y + hh), (w, h)), self.capacity));
    }
}

#[inline]
fn half<T: Primitive + Show>(n: T) -> T {
    n.div(&NumCast::from(2u).unwrap())
}
