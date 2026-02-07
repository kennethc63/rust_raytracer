//vec3.rs

#[derive(Debug)]        //Allow printing
pub struct Vec3 {
    x : f64,
    y : f64,
    z: f64
}

impl Vec3 {
    pub fn new(x : f64, y : f64, z :f64) -> Vec3 {
        Vec3 {x, y, z}
    }
    pub fn zero() -> Vec3{
        Vec3::new(0.0, 0.0, 0.0)
    }
}

//Type alias
pub type Point3=Vec3;
