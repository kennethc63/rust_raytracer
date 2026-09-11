// util.rs
// relies on fastrand

pub fn random_f64() -> f64 {
    fastrand::f64() // Generate random number in range 0 .. 1.0
}

pub fn random_f64_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_f64()
}
