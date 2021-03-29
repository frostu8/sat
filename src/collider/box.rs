use na::{Vector2, zero};

use std::fmt::{Debug, Formatter, Result as FmtResult};

use crate::Polygon;

pub struct BoxCollider {
    origin: Vector2::<f64>,
    size: Vector2::<f64>,
    vertices: [Vector2::<f64>; 4],
}

impl Polygon for BoxCollider {
    fn vertices(&self) -> &[Vector2::<f64>] {
        &self.vertices
    }
}

impl BoxCollider {
    /// Creates a new [`BoxCollider`], where `x` and `y` are the bottom left
    /// corner of the collider, and `w` and `h` are the width and height of
    /// the collider, respectively.
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> BoxCollider {
        let mut collider = BoxCollider {
            origin: Vector2::new(x, y),
            size: Vector2::new(w, h),
            vertices: [zero(); 4],
        };

        collider.rebuild();
        collider
    }

    fn rebuild(&mut self) {
        self.vertices[0] = self.origin;
        self.vertices[1] = self.origin + Vector2::new(0., self.size.y);
        self.vertices[2] = self.origin + self.size;
        self.vertices[3] = self.origin + Vector2::new(self.size.x, 0.);
    }
}

impl Debug for BoxCollider {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "BoxCollider {{x: {}, y: {}, w: {}, h: {}}}",
            self.origin.x,
            self.origin.y,
            self.size.x,
            self.size.y,
        )
    }
}
