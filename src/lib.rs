//! Separating Axis Theorem collision detection.
//!
//! ```
//! use sat::collider::BoxCollider;
//! use sat::Polygon as _;
//!
//! let collider1 = BoxCollider::new(0., 0., 1., 2.);
//! let collider2 = BoxCollider::new(0.5, 0.5, 1., 1.);
//!
//! assert!(collider1.overlap(&collider2));
//! ```

pub mod collider;

pub extern crate nalgebra as na;

use na::Vector2;

/// The [`Polygon`] trait abstracts structs to the SAT algorithm.
///
/// Anyting implementing this trait must return a convex polygon in
/// [`Polygon::vertices`]. Concave shapes must be broken into convex ones.
pub trait Polygon 
where Self: Sized {
    fn vertices(&self) -> &[Vector2::<f64>];

    /// Tests if two polygons are overlapping (colliding).
    fn overlap<T>(&self, other: &T) -> bool
    where T: Polygon {
        axis(self).chain(axis(other))
            .all(|axis| {
                // do sat for each axis
                let self_proj = Projection::project(&axis, self);
                let other_proj = Projection::project(&axis, other);

                self_proj.overlap(&other_proj)
            })
    }
}

fn axis<'a, T>(polygon: &'a T) -> impl Iterator<Item = Vector2::<f64>> + 'a
where T: Polygon {
    let verts = polygon.vertices().into_iter();

    verts.clone().zip(verts.cycle().skip(1))
        .map(|(a, b)| a - b)
        .map(|v| Vector2::new(-v.y, v.x))
}

/// A projection of a shape on an axis.
pub struct Projection {
    axis: Vector2::<f64>,
    pub min: f64,
    pub max: f64,
}

impl Projection {
    fn new(axis: &Vector2::<f64>, init: &Vector2::<f64>) -> Projection {
        let init_dot = axis.dot(init);

        Projection {
            axis: axis.clone(),
            min: init_dot, max: init_dot,
        }
    }
    
    fn add(mut self, point: &Vector2::<f64>) -> Projection{
        let dot = self.axis.dot(point);

        if dot < self.min { self.min = dot; }
        if dot > self.max { self.max = dot; }

        self
    }

    /// Tests if two projections are overlapping.
    pub fn overlap(&self, other: &Projection) -> bool {
        self.max > other.min && self.min < other.max
    }

    /// Projects a polygon on a given axis.
    pub fn project<T>(axis: &Vector2::<f64>, polygon: &T) -> Projection
    where T: Polygon {
        let vertices = polygon.vertices();

        if vertices.len() == 0 {
            panic!("Polygons with 0 vertices are not supported!");
        }

        vertices.into_iter().skip(1)
            .fold(Projection::new(axis, &vertices[0]), |p, x| p.add(x))
    }
}
