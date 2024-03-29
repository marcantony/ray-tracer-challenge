use std::{borrow::Borrow, ops::Deref};

use crate::{
    draw::color::Color,
    math::{point::Point3d, vector::NormalizedVec3d},
    scene::{intersect::Intersection, material::Material, ray::Ray},
};

use super::Object;

#[derive(Debug, PartialEq, Clone)]
pub struct Bounds {
    pub minimum: Point3d,
    pub maximum: Point3d,
}

impl Bounds {
    pub fn enumerate(&self) -> [Point3d; 8] {
        let min = &self.minimum;
        let max = &self.maximum;

        [
            Point3d::new(min.x(), min.y(), min.z()),
            Point3d::new(min.x(), min.y(), max.z()),
            Point3d::new(min.x(), max.y(), min.z()),
            Point3d::new(min.x(), max.y(), max.z()),
            Point3d::new(max.x(), min.y(), min.z()),
            Point3d::new(max.x(), min.y(), max.z()),
            Point3d::new(max.x(), max.y(), min.z()),
            Point3d::new(max.x(), max.y(), max.z()),
        ]
    }

    pub fn from_points<P: Borrow<Point3d>>(points: &[P]) -> Option<Self> {
        if points.is_empty() {
            None
        } else {
            let first = &points[0].borrow();
            let (min, max) = points.iter().fold(
                (
                    [first.x(), first.y(), first.z()],
                    [first.x(), first.y(), first.z()],
                ),
                |(mn, mx), p| {
                    let p = p.borrow();
                    (
                        [mn[0].min(p.x()), mn[1].min(p.y()), mn[2].min(p.z())],
                        [mx[0].max(p.x()), mx[1].max(p.y()), mx[2].max(p.z())],
                    )
                },
            );

            Some(Bounds {
                minimum: Point3d::new(min[0], min[1], min[2]),
                maximum: Point3d::new(max[0], max[1], max[2]),
            })
        }
    }

    pub fn from_bounds<B: Borrow<Bounds>>(bounds: &[B]) -> Self {
        let points: Vec<_> = bounds
            .iter()
            .flat_map(|b| {
                let Bounds {
                    minimum: min,
                    maximum: max,
                } = b.borrow();
                [min, max].into_iter()
            })
            .collect();
        Bounds::from_points(points.deref()).unwrap_or(Bounds {
            minimum: Point3d::new(0.0, 0.0, 0.0),
            maximum: Point3d::new(0.0, 0.0, 0.0),
        })
    }
}

impl Default for Bounds {
    fn default() -> Self {
        Self {
            minimum: Point3d::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY),
            maximum: Point3d::new(f64::INFINITY, f64::INFINITY, f64::INFINITY),
        }
    }
}

pub struct Bounded<T> {
    bounds: Bounds,
    child: T,
}

impl<T: Object> Bounded<T> {
    pub fn new(child: T) -> Self {
        Bounded {
            bounds: child.bounds(),
            child,
        }
    }

    fn test(&self, ray: &Ray) -> bool {
        let (xtmin, xtmax) = check_axis(
            self.bounds.minimum.x(),
            self.bounds.maximum.x(),
            ray.origin.x(),
            ray.direction.x(),
        );
        let (ytmin, ytmax) = check_axis(
            self.bounds.minimum.y(),
            self.bounds.maximum.y(),
            ray.origin.y(),
            ray.direction.y(),
        );
        let (ztmin, ztmax) = check_axis(
            self.bounds.minimum.z(),
            self.bounds.maximum.z(),
            ray.origin.z(),
            ray.direction.z(),
        );

        let tmin = xtmin.max(ytmin).max(ztmin);
        let tmax = xtmax.min(ytmax).min(ztmax);

        tmin <= tmax
    }
}

fn check_axis(min: f64, max: f64, origin: f64, speed: f64) -> (f64, f64) {
    let distance_to_min = min - origin;
    let distance_to_max = max - origin;

    let tmin = distance_to_min / speed;
    let tmax = distance_to_max / speed;

    if tmin > tmax {
        (tmax, tmin)
    } else {
        (tmin, tmax)
    }
}

impl<T: Object> Object for Bounded<T> {
    fn material(&self) -> &Material {
        self.child.material()
    }

    fn intersect(&self, ray: &Ray) -> Vec<Intersection<&dyn Object, Color, NormalizedVec3d>> {
        if self.test(ray) {
            self.child.intersect(ray)
        } else {
            Vec::new()
        }
    }

    fn bounds(&self) -> Bounds {
        self.bounds.clone()
    }
}

#[cfg(test)]
mod bounds_tests {
    use super::*;

    #[test]
    fn bounds_can_enumerate_all_points() {
        let b = Bounds {
            minimum: Point3d::new(0.0, 0.0, 0.0),
            maximum: Point3d::new(1.0, 1.0, 1.0),
        };

        let points = b.enumerate();
        let expected = [
            Point3d::new(0.0, 0.0, 0.0),
            Point3d::new(0.0, 0.0, 1.0),
            Point3d::new(0.0, 1.0, 0.0),
            Point3d::new(0.0, 1.0, 1.0),
            Point3d::new(1.0, 0.0, 0.0),
            Point3d::new(1.0, 0.0, 1.0),
            Point3d::new(1.0, 1.0, 0.0),
            Point3d::new(1.0, 1.0, 1.0),
        ];

        assert_eq!(points, expected);
    }

    #[test]
    fn trying_to_create_bounds_from_no_points() {
        assert_eq!(None, Bounds::from_points::<Point3d>(&vec![]));
    }

    #[test]
    fn creating_bounds_from_one_point() {
        assert_eq!(
            Some(Bounds {
                minimum: Point3d::new(0.0, 1.0, 2.0),
                maximum: Point3d::new(0.0, 1.0, 2.0)
            }),
            Bounds::from_points(&vec![Point3d::new(0.0, 1.0, 2.0)])
        );
    }

    #[test]
    fn creating_bounds_from_many_points() {
        let points = vec![
            Point3d::new(-1.0, 0.0, 0.2),
            Point3d::new(0.0, 5.0, 2.0),
            Point3d::new(-10.0, 0.0, 0.5),
            Point3d::new(0.0, 0.0, 1.0),
        ];

        assert_eq!(
            Bounds::from_points(&points),
            Some(Bounds {
                minimum: Point3d::new(-10.0, 0.0, 0.2),
                maximum: Point3d::new(0.0, 5.0, 2.0),
            })
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        math::{point::Point3d, vector::Vec3d},
        scene::object::test_utils::MockObject,
    };

    use super::*;

    #[test]
    fn bounds_of_bounded_object_are_bounds_of_child() {
        let shape = MockObject::default();
        let shape_bounds = shape.bounds();
        let bounded = Bounded::new(shape);

        assert_eq!(shape_bounds, bounded.bounds());
    }

    #[test]
    fn return_child_intersection_when_boundary_test_passes() {
        let ray = Ray::new(Point3d::new(1.0, 0.0, 0.0), Vec3d::new(1.0, 2.0, 3.0));
        let shape = MockObject {
            intersect_local_arg_expectation: Some(ray.clone()),
            ..Default::default()
        };
        let bounded = Bounded::new(shape);

        let bounded_intersect = bounded.intersect(&ray);
        let child_intersect = bounded.child.intersect(&ray);

        assert_eq!(bounded_intersect.len(), 1);
        assert_eq!(bounded_intersect.len(), child_intersect.len());
        assert!(bounded_intersect[0] == child_intersect[0]);
        assert_eq!(bounded_intersect[0].color, child_intersect[0].color);
        assert_eq!(bounded_intersect[0].normal, child_intersect[0].normal);
    }

    #[test]
    fn material_of_bounded_object_is_material_of_child() {
        let shape = MockObject::default();
        let bounded = Bounded::new(shape);

        assert!(bounded.material() == bounded.child.material());
    }

    mod boundary_test {
        use super::*;

        macro_rules! boundary_intersect_tests {
            ($($name:ident: $value:expr),*) => {
                $(
                    #[test]
                    fn $name() {
                        let (ray, expected) = $value;

                        let shape = MockObject {
                            bounds: Bounds {
                                minimum: Point3d::new(2.0, 2.0, 2.0),
                                maximum: Point3d::new(4.0, 4.0, 4.0),
                            },
                            ..Default::default()
                        };
                        let bounded = Bounded::new(shape);

                        assert_eq!(bounded.test(&ray), expected);
                    }
                )*
            };
        }

        boundary_intersect_tests! {
            a_ray_intersects_pos_x: (Ray::new(Point3d::new(5.0, 3.0, 3.0), Vec3d::new(-1.0, 0.0, 0.0)), true),
            a_ray_intersects_neg_x: (Ray::new(Point3d::new(-5.0, 3.0, 3.0), Vec3d::new(1.0, 0.0, 0.0)), true),
            a_ray_intersects_pos_y: (Ray::new(Point3d::new(3.0, 5.0, 3.0), Vec3d::new(0.0, -1.0, 0.0)), true),
            a_ray_intersects_neg_y: (Ray::new(Point3d::new(3.0, -5.0, 3.0), Vec3d::new(0.0, 1.0, 0.0)), true),
            a_ray_intersects_pos_z: (Ray::new(Point3d::new(3.0, 3.0, 5.0), Vec3d::new(0.0, 0.0,-1.0)), true),
            a_ray_intersects_neg_z: (Ray::new(Point3d::new(3.0, 3.0, -5.0), Vec3d::new(0.0, 0.0, 1.0)), true),
            a_ray_intersects_inside: (Ray::new(Point3d::new(3.0, 3.0, 3.0), Vec3d::new(0.0, 0.0, 1.0)), true),
            a_ray_hits_the_edge: (Ray::new(Point3d::new(5.0, 4.0, 4.0), Vec3d::new(-1.0, 0.0, 0.0)), true)
        }

        boundary_intersect_tests! {
            a_ray_misses_pos_x_1: (Ray::new(Point3d::new(5.0, 4.1, 3.0), Vec3d::new(-1.0, 0.0, 0.0)), false),
            a_ray_misses_pos_x_2: (Ray::new(Point3d::new(5.0, 1.9, 3.0), Vec3d::new(-1.0, 0.0, 0.0)), false),
            a_ray_misses_pos_x_3: (Ray::new(Point3d::new(5.0, 3.0, 4.1), Vec3d::new(-1.0, 0.0, 0.0)), false),
            a_ray_misses_pos_x_4: (Ray::new(Point3d::new(5.0, 3.0, 1.9), Vec3d::new(-1.0, 0.0, 0.0)), false),
            a_ray_misses_neg_x_1: (Ray::new(Point3d::new(-5.0, 4.1, 3.0), Vec3d::new(1.0, 0.0, 0.0)), false),
            a_ray_misses_neg_x_2: (Ray::new(Point3d::new(-5.0, 1.9, 3.0), Vec3d::new(1.0, 0.0, 0.0)), false),
            a_ray_misses_neg_x_3: (Ray::new(Point3d::new(-5.0, 3.0, 4.1), Vec3d::new(1.0, 0.0, 0.0)), false),
            a_ray_misses_neg_x_4: (Ray::new(Point3d::new(-5.0, 3.0, 1.9), Vec3d::new(1.0, 0.0, 0.0)), false),

            a_ray_misses_pos_y_1: (Ray::new(Point3d::new(4.1, 5.0, 3.0), Vec3d::new(0.0, -1.0, 0.0)), false),
            a_ray_misses_pos_y_2: (Ray::new(Point3d::new(1.9, 5.0, 3.0), Vec3d::new(0.0, -1.0, 0.0)), false),
            a_ray_misses_pos_y_3: (Ray::new(Point3d::new(3.0, 5.0, 4.1), Vec3d::new(0.0, -1.0, 0.0)), false),
            a_ray_misses_pos_y_4: (Ray::new(Point3d::new(3.0, 5.0, 1.9), Vec3d::new(0.0, -1.0, 0.0)), false),
            a_ray_misses_neg_y_1: (Ray::new(Point3d::new(4.1, -5.0, 3.0), Vec3d::new(0.0, 1.0, 0.0)), false),
            a_ray_misses_neg_y_2: (Ray::new(Point3d::new(1.9, -5.0, 3.0), Vec3d::new(0.0, 1.0, 0.0)), false),
            a_ray_misses_neg_y_3: (Ray::new(Point3d::new(3.0, -5.0, 4.1), Vec3d::new(0.0, 1.0, 0.0)), false),
            a_ray_misses_neg_y_4: (Ray::new(Point3d::new(3.0, -5.0, 1.9), Vec3d::new(0.0, 1.0, 0.0)), false),

            a_ray_misses_pos_z_1: (Ray::new(Point3d::new(4.1, 3.0, 5.0), Vec3d::new(0.0, 0.0, -1.0)), false),
            a_ray_misses_pos_z_2: (Ray::new(Point3d::new(1.9, 3.0, 5.0), Vec3d::new(0.0, 0.0, -1.0)), false),
            a_ray_misses_pos_z_3: (Ray::new(Point3d::new(3.0, 4.1, 5.0), Vec3d::new(0.0, 0.0, -1.0)), false),
            a_ray_misses_pos_z_4: (Ray::new(Point3d::new(3.0, 1.9, 5.0), Vec3d::new(0.0, 0.0, -1.0)), false),
            a_ray_misses_neg_z_1: (Ray::new(Point3d::new(4.1, 3.0, -5.0), Vec3d::new(0.0, 0.0, 1.0)), false),
            a_ray_misses_neg_z_2: (Ray::new(Point3d::new(1.9, 3.0, -5.0), Vec3d::new(0.0, 0.0, 1.0)), false),
            a_ray_misses_neg_z_3: (Ray::new(Point3d::new(3.0, 4.1, -5.0), Vec3d::new(0.0, 0.0, 1.0)), false),
            a_ray_misses_neg_z_4: (Ray::new(Point3d::new(3.0, 1.9, -5.0), Vec3d::new(0.0, 0.0, 1.0)), false)
        }

        #[test]
        fn a_ray_intersects_a_bounding_box_going_to_infinity() {
            let shape = MockObject {
                bounds: Bounds {
                    minimum: Point3d::new(-1.0, f64::NEG_INFINITY, -1.0),
                    maximum: Point3d::new(1.0, f64::INFINITY, 1.0),
                },
                ..Default::default()
            };
            let bounded = Bounded::new(shape);
            let ray = Ray::new(Point3d::new(5.0, 1e100, 0.0), Vec3d::new(-1.0, 0.0, 0.0));

            assert!(bounded.test(&ray))
        }
    }
}
