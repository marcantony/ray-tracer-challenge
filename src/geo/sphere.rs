use crate::math::tuple::Tuple3;

use super::ray::Ray;

pub struct Sphere();

impl Sphere {
    pub fn new() -> Self {
        Sphere()
    }

    pub fn intersect(&self, r: &Ray) -> Option<[f64; 2]> {
        let sphere_to_ray = r.origin() - &Tuple3::point(0.0, 0.0, 0.0);

        let a = r.direction().dot(r.direction());
        let b = 2.0 * r.direction().dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            None
        } else {
            let disc_sqrt = f64::sqrt(discriminant);
            let t1 = (-b - disc_sqrt) / (2.0 * a);
            let t2 = (-b + disc_sqrt) / (2.0 * a);

            Some([t1, t2])
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{math::tuple::Tuple3, geo::ray::Ray};

    use super::*;

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(Tuple3::point(0.0, 0.0, -5.0), Tuple3::vec(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(&r);

        assert_eq!(xs, Some([4.0, 6.0]));
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(Tuple3::point(0.0, 1.0, -5.0), Tuple3::vec(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(&r);

        assert_eq!(xs, Some([5.0, 5.0]));
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = Ray::new(Tuple3::point(0.0, 2.0, -5.0), Tuple3::vec(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(&r);

        assert_eq!(xs, None);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray::new(Tuple3::point(0.0, 0.0, 0.0), Tuple3::vec(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(&r);

        assert_eq!(xs, Some([-1.0, 1.0]));
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray::new(Tuple3::point(0.0, 0.0, 5.0), Tuple3::vec(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(&r);

        assert_eq!(xs, Some([-6.0, -4.0]));
    }
}