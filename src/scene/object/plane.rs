use crate::{
    math::{matrix::InvertibleMatrix, point::Point3d, vector::NormalizedVec3d},
    scene::{material::Material, ray::Ray},
};

use super::Object;

/// A plane: by default, a plane in xz
pub struct Plane {
    pub transform: InvertibleMatrix<4>,
    pub material: Material,
}

impl Object for Plane {
    fn material(&self) -> &Material {
        &self.material
    }

    fn transform(&self) -> &InvertibleMatrix<4> {
        &self.transform
    }

    fn intersect_local(&self, object_ray: &Ray) -> Vec<f64> {
        // If ray y direction is 0 (epsilon comparison cause floating point)
        if f64::abs(object_ray.direction.y()) < 1e-8 {
            return Vec::new();
        } else {
            vec![-object_ray.origin.y() / object_ray.direction.y()]
        }
    }

    fn normal_at_local(&self, _: &Point3d) -> NormalizedVec3d {
        NormalizedVec3d::new(0.0, 1.0, 0.0).unwrap()
    }
}

impl Default for Plane {
    fn default() -> Self {
        Self {
            transform: InvertibleMatrix::identity(),
            material: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::math::vector::Vec3d;

    use super::*;

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let p: Plane = Default::default();

        let n1 = p.normal_at_local(&Point3d::new(0.0, 0.0, 0.0));
        let n2 = p.normal_at_local(&Point3d::new(10.0, 0.0, -10.0));
        let n3 = p.normal_at_local(&Point3d::new(-5.0, 0.0, 150.0));

        let expected = Vec3d::new(0.0, 1.0, 0.0);
        assert_eq!(*n1, expected);
        assert_eq!(*n2, expected);
        assert_eq!(*n3, expected);
    }

    #[test]
    fn intersect_with_a_ray_parallel_to_the_plane() {
        let p: Plane = Default::default();
        let r = Ray::new(Point3d::new(0.0, 10.0, 0.0), Vec3d::new(0.0, 0.0, 1.0));

        let xs = p.intersect_local(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn intersect_with_a_coplanar_ray() {
        let p: Plane = Default::default();
        let r = Ray::new(Point3d::new(0.0, 0.0, 0.0), Vec3d::new(0.0, 0.0, 1.0));

        let xs = p.intersect_local(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let p: Plane = Default::default();
        let r = Ray::new(Point3d::new(0.0, 1.0, 0.0), Vec3d::new(0.0, -1.0, 0.0));

        let xs = p.intersect_local(&r);
        assert_eq!(xs, vec![1.0]);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let p: Plane = Default::default();
        let r = Ray::new(Point3d::new(0.0, -1.0, 0.0), Vec3d::new(0.0, 1.0, 0.0));

        let xs = p.intersect_local(&r);
        assert_eq!(xs, vec![1.0]);
    }
}
