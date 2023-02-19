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
