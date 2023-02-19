// Simple coordinate object to represent points and vectors
#[derive(Clone, Copy)]
pub struct XY {
    pub x: f32,
    pub y: f32,
}

// Axis-aligned bounding box with half dimension and center
pub struct AABB {
    pub center: XY,
    pub half_dimension: f32,
}

impl AABB {
    pub fn new(x: f32, y: f32, half_dimension: f32) -> AABB {
        AABB {
            center: XY { x, y },
            half_dimension,
        }
    }

    pub fn contains_point(&self, point: &XY) -> bool {
        // Left bound
        point.x > self.center.x -self.half_dimension &&
        // Right Bound
        point.x < self.center.x + self.half_dimension &&
        // Top Bound
        point.y > self.center.y - self.half_dimension &&
        // Bottom Bound
        point.y < self.center.y + self.half_dimension
    }

    pub fn intersects_aabb(&self, other: &AABB) -> bool {
        let x_overlap =
            (self.center.x - other.center.x).abs() <= (self.half_dimension + other.half_dimension);
        let y_overlap =
            (self.center.y - other.center.y).abs() <= (self.half_dimension + other.half_dimension);
        return x_overlap && y_overlap;
    }
}

pub struct QuadTree<T> {
    // TODO maybe make the capacity a const
    capacity: usize,
    pub boundary: AABB,
    points: Vec<XY>,
    points_data: Vec<T>,

    // Children
    pub north_west: Option<Box<QuadTree<T>>>,
    pub north_east: Option<Box<QuadTree<T>>>,
    pub south_west: Option<Box<QuadTree<T>>>,
    pub south_east: Option<Box<QuadTree<T>>>,
}

impl<T> QuadTree<T> {
    pub fn new(capacity: usize, boundary: AABB) -> QuadTree<T> {
        QuadTree {
            capacity,
            boundary,
            points: Vec::with_capacity(capacity),
            points_data: Vec::with_capacity(capacity),
            north_west: Option::None,
            north_east: Option::None,
            south_west: Option::None,
            south_east: Option::None,
        }
    }

    pub fn insert(&mut self, p: XY, data: T) -> bool {
        if self.boundary.contains_point(&p) {
            return self.insert_internal(p, data);
        } else {
            return false;
        }
    }

    pub fn query_range(range: AABB) {}

    // Insert without checking if the point can be inserted.
    pub fn insert_internal(&mut self, p: XY, data: T) -> bool {
        if self.points.len() < self.capacity && self.north_west.is_none() {
            //TODO Try to see how push_within_capacity works.
            self.points.push(p);
            self.points_data.push(data);
            return true;
        }

        if self.north_west.is_none() {
            self.subdivide();
        }

        //TODO find a good way to put this in functions
        match &mut self.north_west {
            Some(qt) => {
                if qt.boundary.contains_point(&p) {
                    return qt.insert_internal(p, data);
                }
            }
            None => {}
        }

        match &mut self.north_east {
            Some(qt) => {
                if qt.boundary.contains_point(&p) {
                    return qt.insert_internal(p, data);
                }
            }
            None => {}
        }

        match &mut self.south_west {
            Some(qt) => {
                if qt.boundary.contains_point(&p) {
                    return qt.insert_internal(p, data);
                }
            }
            None => {}
        }

        match &mut self.south_east {
            Some(qt) => {
                if qt.boundary.contains_point(&p) {
                    return qt.insert_internal(p, data);
                }
            }
            None => {}
        }

        return false;
    }

    fn subdivide(&mut self) {
        self.north_west = Some(Box::new(QuadTree::new(
            self.capacity,
            AABB {
                center: XY {
                    x: self.boundary.center.x - self.boundary.half_dimension / 2.0,
                    y: self.boundary.center.y - self.boundary.half_dimension / 2.0,
                },
                half_dimension: self.boundary.half_dimension / 2.0,
            },
        )));
        self.north_east = Some(Box::new(QuadTree::new(
            self.capacity,
            AABB {
                center: XY {
                    x: self.boundary.center.x + self.boundary.half_dimension / 2.0,
                    y: self.boundary.center.y - self.boundary.half_dimension / 2.0,
                },
                half_dimension: self.boundary.half_dimension / 2.0,
            },
        )));
        self.south_west = Some(Box::new(QuadTree::new(
            self.capacity,
            AABB {
                center: XY {
                    x: self.boundary.center.x - self.boundary.half_dimension / 2.0,
                    y: self.boundary.center.y + self.boundary.half_dimension / 2.0,
                },
                half_dimension: self.boundary.half_dimension / 2.0,
            },
        )));
        self.south_east = Some(Box::new(QuadTree::new(
            self.capacity,
            AABB {
                center: XY {
                    x: self.boundary.center.x + self.boundary.half_dimension / 2.0,
                    y: self.boundary.center.y + self.boundary.half_dimension / 2.0,
                },
                half_dimension: self.boundary.half_dimension / 2.0,
            },
        )));
    }
}

#[cfg(test)]
mod tests {
    use crate::quadtree::AABB;
    use crate::quadtree::XY;

    #[test]
    fn test_intersects_aabb() {
        let box1 = AABB {
            center: XY { x: 0.0, y: 0.0 },
            half_dimension: 1.0,
        };
        let box2 = AABB {
            center: XY { x: 1.5, y: 1.5 },
            half_dimension: 1.0,
        };
        let overlap = box1.intersects_aabb(&box2);
        assert_eq!(overlap, true);

        let box3 = AABB {
            center: XY { x: 0.0, y: 0.0 },
            half_dimension: 1.0,
        };
        let box4 = AABB {
            center: XY { x: 3.0, y: 3.0 },
            half_dimension: 1.0,
        };
        let no_overlap = box3.intersects_aabb(&box4);
        assert_eq!(no_overlap, false);
    }

    #[test]
    fn test_contains_point() {
        let box1 = AABB {
            center: XY { x: 0.0, y: 0.0 },
            half_dimension: 1.0,
        };

        // Containing Points
        assert_eq!(box1.contains_point(&XY { x: 0.0, y: 0.0 }), true);
        assert_eq!(box1.contains_point(&XY { x: 0.9, y: -0.5 }), true);
        assert_eq!(box1.contains_point(&XY { x: -0.5, y: 0.5 }), true);

        // Not Containing Points
        assert_eq!(box1.contains_point(&XY { x: 2.0, y: 1.0 }), false);
        assert_eq!(box1.contains_point(&XY { x: 1.9, y: -2.5 }), false);
        assert_eq!(box1.contains_point(&XY { x: -1.5, y: 3.5 }), false);
    }
}
