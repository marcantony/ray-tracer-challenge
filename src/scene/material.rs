use crate::{
    draw::color::{self, Color},
    math::tuple::Tuple3,
};

use super::light::PointLight;

#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

pub fn lighting(
    material: &Material,
    point: &Tuple3,
    light: &PointLight,
    eyev: &Tuple3,
    normalv: &Tuple3,
) -> Color {
    let effective_color = &material.color * &light.intensity;
    let lightv = (&light.position - &point).norm();

    let ambient = &effective_color * material.ambient;

    let light_dot_normal = lightv.dot(normalv);

    let (diffuse, specular) = if light_dot_normal < 0.0 {
        (color::black(), color::black())
    } else {
        let diff = &(&effective_color * material.diffuse) * light_dot_normal;

        let reflectv = -&lightv.reflect(&normalv);
        let reflect_dot_eye = reflectv.dot(&eyev);

        (
            diff,
            if reflect_dot_eye <= 0.0 {
                color::black()
            } else {
                let factor = reflect_dot_eye.powf(material.shininess);
                &(&light.intensity * material.specular) * factor
            },
        )
    };

    return &(&ambient + &diffuse) + &specular;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_material() {
        let m: Material = Default::default();

        assert_eq!(m.color, Color::new(1.0, 1.0, 1.0));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    mod lighting {
        use crate::{math::tuple::Tuple3, scene::light::PointLight};

        use super::*;

        fn setup() -> (Material, Tuple3) {
            (Default::default(), Tuple3::point(0.0, 0.0, 0.0))
        }

        #[test]
        fn lighting_with_eye_between_light_and_surface() {
            let (m, position) = setup();
            let eyev = Tuple3::vec(0.0, 0.0, -1.0);
            let normalv = Tuple3::vec(0.0, 0.0, -1.0);
            let light = PointLight {
                position: Tuple3::point(0.0, 0.0, -10.0),
                intensity: Color::new(1.0, 1.0, 1.0),
            };

            let result = lighting(&m, &position, &light, &eyev, &normalv);
            assert_eq!(result, Color::new(1.9, 1.9, 1.9));
        }

        #[test]
        fn lighting_with_eye_between_light_and_surface_eye_offset_45_degrees() {
            let (m, position) = setup();
            let t = std::f64::consts::SQRT_2 / 2.0;
            let eyev = Tuple3::vec(0.0, t, -t);
            let normalv = Tuple3::vec(0.0, 0.0, -1.0);
            let light = PointLight {
                position: Tuple3::point(0.0, 0.0, -10.0),
                intensity: Color::new(1.0, 1.0, 1.0),
            };

            let result = lighting(&m, &position, &light, &eyev, &normalv);
            assert_eq!(result, Color::new(1.0, 1.0, 1.0));
        }

        #[test]
        fn lighting_with_eye_opposite_surface_light_offset_45_degrees() {
            let (m, position) = setup();
            let eyev = Tuple3::vec(0.0, 0.0, -1.0);
            let normalv = Tuple3::vec(0.0, 0.0, -1.0);
            let light = PointLight {
                position: Tuple3::point(0.0, 10.0, -10.0),
                intensity: Color::new(1.0, 1.0, 1.0),
            };

            let result = lighting(&m, &position, &light, &eyev, &normalv);
            assert_color_approx_equals(&result, &Color::new(0.7364, 0.7364, 0.7364));
        }

        #[test]
        fn lighting_with_eye_in_path_of_reflection_vector() {
            let (m, position) = setup();
            let t = std::f64::consts::SQRT_2 / 2.0;
            let eyev = Tuple3::vec(0.0, -t, -t);
            let normalv = Tuple3::vec(0.0, 0.0, -1.0);
            let light = PointLight {
                position: Tuple3::point(0.0, 10.0, -10.0),
                intensity: Color::new(1.0, 1.0, 1.0),
            };

            let result = lighting(&m, &position, &light, &eyev, &normalv);
            assert_color_approx_equals(&result, &Color::new(1.6364, 1.6364, 1.6364));
        }

        #[test]
        fn lighting_with_light_behind_surface() {
            let (m, position) = setup();
            let eyev = Tuple3::vec(0.0, 0.0, -1.0);
            let normalv = Tuple3::vec(0.0, 0.0, -1.0);
            let light = PointLight {
                position: Tuple3::point(0.0, 0.0, 10.0),
                intensity: Color::new(1.0, 1.0, 1.0),
            };

            let result = lighting(&m, &position, &light, &eyev, &normalv);
            assert_eq!(result, Color::new(0.1, 0.1, 0.1));
        }
    }

    fn assert_color_approx_equals(a: &Color, b: &Color) {
        let tolerance = 1e-4;
        assert!(equal_with_tolerance(a.r(), b.r(), tolerance));
        assert!(equal_with_tolerance(a.g(), b.g(), tolerance));
        assert!(equal_with_tolerance(a.b(), b.b(), tolerance));
    }

    fn equal_with_tolerance(a: f64, b: f64, tolerance: f64) -> bool {
        f64::abs(a - b) <= tolerance
    }
}