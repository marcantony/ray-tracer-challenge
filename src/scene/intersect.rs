use crate::math::{point::Point3d, util, vector::NormalizedVec3d};

use super::{ray::Ray, sphere::Sphere};

const SHADOW_BIAS: f64 = 1e-5;

#[derive(Debug, Clone)]
pub struct Intersection<'a> {
    t: f64,
    object: &'a Sphere,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Precomputation<'a> {
    pub t: f64,
    pub object: &'a Sphere,
    pub point: Point3d,
    pub eye_v: NormalizedVec3d,
    pub normal_v: NormalizedVec3d,
    pub inside: bool,
    pub over_point: Point3d,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, object: &Sphere) -> Intersection {
        Intersection { t, object }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn object(&self) -> &Sphere {
        self.object
    }

    pub fn prepare_computations(&self, ray: &Ray) -> Precomputation {
        let t = self.t();
        let object = self.object();
        let point = ray.position(t);
        let eye_v = NormalizedVec3d::try_from(-ray.direction()).unwrap();
        let normal_v = object.normal_at(&point);

        let normal_dot_eye = normal_v.dot(&eye_v);
        let (adjusted_normal_v, inside) = if normal_dot_eye < 0.0 {
            (-normal_v, true)
        } else {
            (normal_v, false)
        };

        let over_point = &point + &(&*adjusted_normal_v * SHADOW_BIAS);

        Precomputation {
            t,
            object,
            point,
            eye_v,
            normal_v: adjusted_normal_v,
            inside,
            over_point,
        }
    }
}

impl<'a> PartialEq for Intersection<'a> {
    fn eq(&self, other: &Self) -> bool {
        util::are_equal(self.t, other.t) && self.object == other.object
    }
}

pub fn hit<'a, 'b>(intersections: &'a [Intersection<'b>]) -> Option<&'a Intersection<'b>> {
    intersections.iter().fold(None, |acc, i| {
        if i.t() >= 0.0 {
            acc.map(|lowest| if lowest.t() < i.t() { lowest } else { i })
                .or(Some(i))
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod test {
    use crate::math::{point::Point3d, vector::Vec3d};

    use super::*;

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = Sphere::unit();

        let i = Intersection::new(3.5, &s);

        assert_eq!(i.t(), 3.5);
        assert_eq!(i.object(), &s);
    }

    mod hit {
        use super::*;

        #[test]
        fn hit_when_all_intersections_have_positive_t() {
            let s = Sphere::unit();
            let i1 = Intersection::new(1.0, &s);
            let i2 = Intersection::new(2.0, &s);
            let xs = vec![i2.clone(), i1.clone()];

            let i = hit(&xs);

            assert_eq!(i, Some(&i1));
        }

        #[test]
        fn hit_when_some_intersections_have_negative_t() {
            let s = Sphere::unit();
            let i1 = Intersection::new(-1.0, &s);
            let i2 = Intersection::new(1.0, &s);
            let xs = vec![i2.clone(), i1.clone()];

            let i = hit(&xs);

            assert_eq!(i, Some(&i2));
        }

        #[test]
        fn hit_when_all_intersections_have_negative_t() {
            let s = Sphere::unit();
            let i1 = Intersection::new(-2.0, &s);
            let i2 = Intersection::new(-1.0, &s);
            let xs = vec![i2.clone(), i1.clone()];

            let i = hit(&xs);

            assert_eq!(i, None);
        }

        #[test]
        fn hit_is_always_lowest_nonnegative_intersection() {
            let s = Sphere::unit();
            let i1 = Intersection::new(5.0, &s);
            let i2 = Intersection::new(7.0, &s);
            let i3 = Intersection::new(-3.0, &s);
            let i4 = Intersection::new(2.0, &s);
            let xs = vec![i1.clone(), i2.clone(), i3.clone(), i4.clone()];

            let i = hit(&xs);

            assert_eq!(i, Some(&i4));
        }
    }

    mod prepare_computations {
        use crate::{math::matrix::InvertibleMatrix, scene::transformation};

        use super::*;

        #[test]
        fn precomputing_the_state_of_an_intersection() {
            let r = Ray::new(Point3d::new(0.0, 0.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));
            let s: Sphere = Default::default();
            let i = Intersection::new(4.0, &s);

            let comps = i.prepare_computations(&r);

            assert_eq!(comps.t, i.t());
            assert_eq!(comps.object, i.object());
            assert_eq!(comps.point, Point3d::new(0.0, 0.0, -1.0));
            assert_eq!(
                comps.eye_v,
                NormalizedVec3d::try_from(Vec3d::new(0.0, 0.0, -1.0)).unwrap()
            );
            assert_eq!(
                comps.normal_v,
                NormalizedVec3d::try_from(Vec3d::new(0.0, 0.0, -1.0)).unwrap()
            );
        }

        #[test]
        fn hit_when_an_intersection_occurs_on_the_outside() {
            let r = Ray::new(Point3d::new(0.0, 0.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));
            let s: Sphere = Default::default();
            let i = Intersection { t: 4.0, object: &s };

            let comps = i.prepare_computations(&r);

            assert_eq!(comps.inside, false);
        }

        #[test]
        fn hit_when_an_intersection_occurs_on_the_inside() {
            let r = Ray::new(Point3d::new(0.0, 0.0, 0.0), Vec3d::new(0.0, 0.0, 1.0));
            let s: Sphere = Default::default();
            let i = Intersection { t: 1.0, object: &s };

            let comps = i.prepare_computations(&r);

            assert_eq!(comps.point, Point3d::new(0.0, 0.0, 1.0));
            assert_eq!(
                comps.eye_v,
                NormalizedVec3d::try_from(Vec3d::new(0.0, 0.0, -1.0)).unwrap()
            );
            assert_eq!(comps.inside, true);
            assert_eq!(
                comps.normal_v,
                NormalizedVec3d::try_from(Vec3d::new(0.0, 0.0, -1.0)).unwrap()
            );
        }

        #[test]
        fn the_hit_should_offset_the_point() {
            let r = Ray {
                origin: Point3d::new(0.0, 0.0, -5.0),
                direction: Vec3d::new(0.0, 0.0, 1.0),
            };
            let shape = Sphere {
                transform: InvertibleMatrix::try_from(transformation::translation(0.0, 0.0, 1.0))
                    .unwrap(),
                ..Default::default()
            };
            let i = Intersection::new(5.0, &shape);

            let comps = i.prepare_computations(&r);

            assert!(comps.over_point.z() < -SHADOW_BIAS / 2.0);
            assert!(comps.point.z() > comps.over_point.z());
        }
    }
}
