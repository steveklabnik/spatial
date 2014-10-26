extern crate spatial;

use spatial::quadtree::{Quadtree, Index, Volume};

#[deriving(Clone)]
struct Object {
    x: f32,
    y: f32
}

impl Object {
    pub fn new(x: f32, y: f32) -> Object {
        Object {
            x: x,
            y: y
        }
    }
}

impl Index<f32> for Object {
    fn quadtree_index(&self) -> [f32, ..2] {
        [self.x, self.y]
    }
}


#[test]
fn main() {
    let vol = Volume::new([0.0, 0.0], [1.0, 1.0]);
    let mut tree = Quadtree::new(vol);
    
    assert_eq!(tree.insert(Object::new(0.25, 0.25)), true);
    assert_eq!(tree.insert(Object::new(0.75, 0.25)), true);
    assert_eq!(tree.insert(Object::new(0.25, 0.75)), true);
    assert_eq!(tree.insert(Object::new(0.75, 0.75)), true);
    
    assert_eq!(tree.get_in_volume(&Volume::new([0.0, 0.0], [0.5, 0.5])).len(), 1);
    assert_eq!(tree.get_in_volume(&Volume::new([0.5, 0.0], [1.0, 0.5])).len(), 1);
    assert_eq!(tree.get_in_volume(&Volume::new([0.0, 0.5], [0.5, 1.0])).len(), 1);
    assert_eq!(tree.get_in_volume(&Volume::new([0.5, 0.5], [1.0, 1.0])).len(), 1);
    
    assert_eq!(tree.len(), 4);
}
