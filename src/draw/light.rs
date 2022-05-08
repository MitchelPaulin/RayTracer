use crate::math::tuples::Tuple;

use super::{color::Color, material::Material};

pub struct PointLight {
    intensity: Color,
    pub position: Tuple,
}

impl PointLight {
    pub fn new(intensity: Color, position: Tuple) -> PointLight {
        assert!(position.is_point());
        PointLight {
            intensity,
            position,
        }
    }

    /*
        Implementation of the Phong reflection model
    */
    pub fn lighting(
        &self,
        material: &Material,
        position: Tuple,
        eyev: Tuple,
        normalv: Tuple,
        is_shadow: bool,
    ) -> Color {
        // combine the surface color with the lights color/intensity
        let effective_color = material.pattern.get_color_at(&position) * self.intensity;

        // find the direction to the light source
        let lightv = (self.position - position).normalize();

        /*
           Compute the ambient contribution which is light from other objects
           in the scene, for out purposes we just have this as a constant
        */
        let ambient = effective_color * material.ambient;

        /*
            light_dot_normal represents the cosine of the angle between the
            light vector and the normal vector. A negative number means the
            light is on the other side of the surface
        */
        let light_dot_normal = lightv.dot(&normalv);
        let diffuse;
        let specular;

        if is_shadow || light_dot_normal < 0.0 {
            // light is behind shape or there is another object between it and the source, no contribution to final color
            diffuse = Color::black();
            specular = Color::black();
        } else {
            // compute the diffuse contribution, the light spreading over the surface
            diffuse = effective_color * material.diffuse * light_dot_normal;

            /*
                reflect_dot_eye represents the cosine of th angle between the
                reflection vector and the eye vector. A negative number means
                the light reflects away from the eye
            */
            let reflectv = (-lightv).reflect(&normalv);
            let reflect_dot_eye = reflectv.dot(&eyev);

            if reflect_dot_eye <= 0.0 {
                specular = Color::black();
            } else {
                /*
                    Compute the specular contribution, this is the bright dot
                    reflection on the shape from the light itself
                */
                let factor = reflect_dot_eye.powf(material.shininess);
                specular = self.intensity * material.specular * factor;
            }
        }

        // add the three contributions together to get the final shading
        ambient + diffuse + specular
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn eye_between_light_and_surface() {
        let position = Tuple::point(0.0, 0.0, 0.0);
        let m = Material::default_material();

        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Tuple::point(0.0, 0.0, -10.0));
        let res = light.lighting(&m, position, eyev, normalv, false);
        assert!(res == Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn eye_between_light_and_surface_offset_45() {
        let position = Tuple::point(0.0, 0.0, 0.0);
        let m = Material::default_material();

        let eyev = Tuple::vector(0.0, (2.0_f64).sqrt() / 2.0, (2.0_f64).sqrt() / -2.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Tuple::point(0.0, 0.0, -10.0));
        let res = light.lighting(&m, position, eyev, normalv, false);
        assert!(res == Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn eye_opposite_surface_light_offset_45() {
        let position = Tuple::point(0.0, 0.0, 0.0);
        let m = Material::default_material();

        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Tuple::point(0.0, 10.0, -10.0));
        let res = light.lighting(&m, position, eyev, normalv, false);
        assert!(res == Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_with_shadow() {
        let position = Tuple::point(0.0, 0.0, 0.0);
        let m = Material::default_material();

        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Tuple::point(0.0, 0.0, -10.0));
        let res = light.lighting(&m, position, eyev, normalv, true);
        assert!(res == Color::new(0.1, 0.1, 0.1));
    }
}
