//! Separating Axis Theorem collision detection.

pub mod collider;

use nalgebra::Vector2;

pub trait Polygon 
where Self: Sized {
    fn vertices(&self) -> &[Vector2::<f64>];

    fn overlap<T>(&self, other: &T) -> bool
    where T: Polygon {
        axis(self, other)
            .all(|axis| {
                // do sat for each axis
                let self_proj = Projection::project(&axis, self);
                let other_proj = Projection::project(&axis, other);

                self_proj.overlap(&other_proj)
            })
    }
}

fn axis<'a, T, U>(lhs: &'a T, rhs: &'a U) -> impl Iterator<Item = Vector2::<f64>> + 'a
where T: Polygon, U: Polygon {
    axis_single(lhs).chain(axis_single(rhs))
}

fn axis_single<'a, T>(polygon: &'a T) -> impl Iterator<Item = Vector2::<f64>> + 'a
where T: Polygon {
    let verts = polygon.vertices().into_iter();

    verts.clone().zip(verts.cycle().skip(1))
        .map(|(a, b)| a - b)
        .map(|v| Vector2::new(-v.y, v.x))
}

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

    pub fn overlap(&self, other: &Projection) -> bool {
        self.max > other.min && self.min < other.max
    }

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