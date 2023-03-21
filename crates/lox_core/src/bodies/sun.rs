// Auto-generated by `lox_gen`. Do not edit!
use super::{Ellipsoid, NaifId, PointMass, Spheroid};
pub struct Sun;
impl NaifId for Sun {
    fn id() -> i32 {
        10i32
    }
}
impl PointMass for Sun {
    fn gravitational_parameter() -> f64 {
        132712440041.27942f64
    }
}
impl Ellipsoid for Sun {
    fn polar_radius() -> f64 {
        695700f64
    }
    fn mean_radius() -> f64 {
        695700f64
    }
}
impl Spheroid for Sun {
    fn equatorial_radius() -> f64 {
        695700f64
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_naif_id_10() {
        assert_eq!(Sun::id(), 10i32)
    }
    #[test]
    fn test_point_mass_10() {
        assert_eq!(Sun::gravitational_parameter(), 132712440041.27942f64);
    }
    #[test]
    fn test_spheroid_10() {
        assert_eq!(Sun::polar_radius(), 695700f64);
        assert_eq!(Sun::mean_radius(), 695700f64);
        assert_eq!(Sun::equatorial_radius(), 695700f64);
    }
}
