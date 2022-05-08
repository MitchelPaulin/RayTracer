use super::{
    color::Color,
    patterns::{Pattern, Solid},
};

pub struct Material {
    pub pattern: Box<dyn Pattern>,
    pub ambient: f64,    // between 0 and 1
    pub diffuse: f64,    // between 0 and 1
    pub specular: f64,   // between 0 and 1
    pub shininess: f64,  // between 10 and 200 (large to small)
    pub reflective: f64, // between 0 and 1
}

impl Material {
    pub fn default_material() -> Material {
        Material {
            pattern: Box::new(Solid::new(Color::new(1.0, 1.0, 1.0))),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.0,
        }
    }
}
