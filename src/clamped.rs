use std::ops:: {
    Add,
    Sub,
    Mul,
};
use std::convert::From;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Clamped(i64);
impl Clamped {
    pub fn clamped_value(&self) -> u8{
        let val = self.0;
        if val < 0 {
            0
        } else if val > 255 {
            255
        } else {
            val as u8
        }
    }
    pub fn unclamped_value(&self) -> i64 {
        self.0
    }
}
impl Add for Clamped {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}
impl Sub for Clamped {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}
impl Mul<f64> for Clamped {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        let val = self.0 as f64;
        let out = val * rhs;
        Self(out as i64)
    }
}
impl From<u8> for Clamped {
    fn from(x: u8) -> Self {
        Self(x as i64)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamped_value_is_correct() {
        assert_eq!(0, Clamped(0).clamped_value());
        assert_eq!(0, Clamped(-1).clamped_value());
        assert_eq!(0, Clamped(-3221930293).clamped_value());

        assert_eq!(1, Clamped(1).clamped_value());
        assert_eq!(154, Clamped(154).clamped_value());
        assert_eq!(255, Clamped(255).clamped_value());

        assert_eq!(255, Clamped(256).clamped_value());
        assert_eq!(255, Clamped(1023).clamped_value());
        assert_eq!(255, Clamped(29313293).clamped_value());
    }
}
