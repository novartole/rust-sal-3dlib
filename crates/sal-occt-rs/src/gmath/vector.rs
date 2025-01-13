use super::*;
//
//
impl Vector {
    ///
    /// Returns the normalized vector oX.
    pub fn unit_x() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }
    ///
    /// Returns the normalized vector oY.
    pub fn unit_y() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }
    ///
    /// Returns the normalized vector oZ.
    pub fn unit_z() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }
    ///
    /// Creates a vector from coordinates.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self([x, y, z].into())
    }
}
//
//
impl From<Vector> for gmath::Vector<3> {
    fn from(val: Vector) -> Self {
        val.0
    }
}
