use quadtree::Quadtree;
pub mod quadtree;

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

impl quadtree::Index<f32> for Object {
    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }
}

#[test]
fn test_quadtree() {
    let vol = quadtree::Volume::new((0.0, 0.0), (1.0, 1.0));
    let mut tree = Quadtree::new(vol);

    assert_eq!(tree.insert(Object::new(0.25, 0.25)), true);
    assert_eq!(tree.insert(Object::new(0.75, 0.25)), true);
    assert_eq!(tree.insert(Object::new(0.25, 0.75)), true);
    assert_eq!(tree.insert(Object::new(0.75, 0.75)), true);

    assert_eq!(tree.get_in_volume(quadtree::Volume::new((0.0, 0.0), (0.5, 0.5))).len(), 1);
    assert_eq!(tree.get_in_volume(quadtree::Volume::new((0.5, 0.0), (1.0, 0.5))).len(), 1);
    assert_eq!(tree.get_in_volume(quadtree::Volume::new((0.0, 0.5), (0.5, 1.0))).len(), 1);
    assert_eq!(tree.get_in_volume(quadtree::Volume::new((0.5, 0.5), (1.0, 1.0))).len(), 1);

    assert_eq!(tree.len(), 4);
}
