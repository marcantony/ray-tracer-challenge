use crate::{
    draw::color::Color,
    math::{point::Point3d, vector::NormalizedVec3d},
    scene::{intersect::Intersection, material::Material, ray::Ray},
};

use super::{bounded::Bounds, Object, PhysicalObject};

/// A plane: by default, a plane in xz
#[derive(Default)]
pub struct Plane {
    pub material: Material,
}

impl PhysicalObject for Plane {
    fn normal_at(&self, _: &Point3d) -> NormalizedVec3d {
        NormalizedVec3d::new(0.0, 1.0, 0.0).unwrap()
    }
}

impl Object for Plane {
    fn material(&self) -> &Material {
        &self.material
    }

    fn intersect(
        &self,
        object_ray: &Ray,
    ) -> Vec<Intersection<&dyn Object, Color, NormalizedVec3d>> {
        // If ray y direction is 0 (epsilon comparison cause floating point)
        if f64::abs(object_ray.direction.y()) < 1e-8 {
            Vec::new()
        } else {
            [-object_ray.origin.y() / object_ray.direction.y()]
                .into_iter()
                .map(|t| super::build_basic_intersection(object_ray, t, self))
                .collect()
        }
    }

    fn bounds(&self) -> Bounds {
        Bounds {
            // Use an epsilon for -y/+y to avoid any floating point wonkiness
            minimum: Point3d::new(f64::NEG_INFINITY, -1e8, f64::NEG_INFINITY),
            maximum: Point3d::new(f64::INFINITY, 1e8, f64::INFINITY),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::math::vector::Vec3d;
    use crate::scene::intersect as is;

    use super::*;

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let p: Plane = Default::default();

        let n1 = p.normal_at(&Point3d::new(0.0, 0.0, 0.0));
        let n2 = p.normal_at(&Point3d::new(10.0, 0.0, -10.0));
        let n3 = p.normal_at(&Point3d::new(-5.0, 0.0, 150.0));

        let expected = Vec3d::new(0.0, 1.0, 0.0);
        assert_eq!(*n1, expected);
        assert_eq!(*n2, expected);
        assert_eq!(*n3, expected);
    }

    #[test]
    fn intersect_with_a_ray_parallel_to_the_plane() {
        let p: Plane = Default::default();
        let r = Ray::new(Point3d::new(0.0, 10.0, 0.0), Vec3d::new(0.0, 0.0, 1.0));

        let xs = p.intersect(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn intersect_with_a_coplanar_ray() {
        let p: Plane = Default::default();
        let r = Ray::new(Point3d::new(0.0, 0.0, 0.0), Vec3d::new(0.0, 0.0, 1.0));

        let xs = p.intersect(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let p: Plane = Default::default();
        let r = Ray::new(Point3d::new(0.0, 1.0, 0.0), Vec3d::new(0.0, -1.0, 0.0));

        let xs = is::test_utils::to_ts(&p.intersect(&r));
        assert_eq!(xs, vec![1.0]);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let p: Plane = Default::default();
        let r = Ray::new(Point3d::new(0.0, -1.0, 0.0), Vec3d::new(0.0, 1.0, 0.0));

        let xs = is::test_utils::to_ts(&p.intersect(&r));
        assert_eq!(xs, vec![1.0]);
    }

    #[test]
    fn intersection_returns_color_and_normal_at_point() {
        let r = Ray::new(Point3d::new(0.0, 1.0, 0.0), Vec3d::new(0.0, -1.0, 0.0));
        let plane = Plane::default();

        let xs = plane.intersect(&r);

        for x in xs {
            let p = r.position(x.t());
            let n = plane.normal_at(&p);
            let c = plane.material().surface.color_at(&p);
            assert_eq!(x.normal, n);
            assert_eq!(x.color, c);
        }
    }
}
