use crate::vec2::Vec2;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, Div};


type I = i64;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Vec3 {
    pub x: I,
    pub y: I,
    pub z: I,
}

impl Vec3 {
    pub fn new(x: I, y: I, z: I) -> Self {
        Vec3 { x, y, z }
    }

    pub fn zero() -> Self {
        Vec3 { x: 0, y: 0, z: 0 }
    }

    pub fn one() -> Self {
        Vec3 { x: 1, y: 1, z: 1 }
    }

    pub fn min(&self, other: Self) -> Self {
        Vec3 { x: self.x.min(other.x), y: self.y.min(other.y), z: self.z.min(other.z) }
    }

    pub fn max(&self, other: Self) -> Self {
        Vec3 { x: self.x.max(other.x), y: self.y.max(other.y), z: self.z.max(other.z) }
    }

    pub fn abs(&self) -> Self {
        Vec3 { x: self.x.abs(), y: self.y.abs(), z: self.z.abs() }
    }

    pub fn dot(&self, other: Self) -> i128 {
        self.x as i128 * other.x as i128 + self.y as i128 * other.y as i128 + self.z as i128 * other.z as i128
    }

    pub fn cross(&self, other: Self) -> Self {
        Vec3 { 
            x: (self.y as i128 * other.z as i128 - self.z as i128 * other.y as i128) as I,
            y: (self.z as i128 * other.x as i128 - self.x as i128 * other.z as i128) as I,
            z: (self.x as i128 * other.y as i128 - self.y as i128 * other.x as i128) as I
        }
    }

    pub fn length(&self) -> f64 {
        (self.x as f64 * self.x as f64 + self.y as f64 * self.y as f64 + self.z as f64 * self.z as f64).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let length = self.length();
        Vec3 { x: (self.x as f64 / length) as I, y: (self.y as f64 / length) as I, z: (self.z as f64 / length) as I }
    }

    pub fn pseudo_normalize(&self) -> Self {
        let divisor = num::integer::gcd(self.x, num::integer::gcd(self.y, self.z));
        Vec3 { x: (self.x / divisor) as I, y: (self.y / divisor) as I, z: (self.z / divisor) as I }
    }

    pub fn manhattan(&self) -> I {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    pub fn xy(&self) -> Vec2 {
        Vec2(self.x, self.y)
    }

    pub fn length_squared(&self) -> I {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length_squared_128(&self) -> i128 {
        self.x as i128 * self.x as i128 + self.y as i128 * self.y as i128 + self.z as i128 * self.z as i128
    }

    pub fn try_div(self, rhs: Vec3) -> Result<Vec3, ()> {
        if rhs.x != 0 && rhs.y != 0 && rhs.z != 0  {
            Ok(Vec3 { x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z })
        } else {
            Err(())
        }
    }
}

impl From<[I; 3]> for Vec3
{
    fn from(value: [I; 3]) -> Self {
        Vec3 { x: value[0], y: value[1], z: value[2] }
    }
}

impl From<&[I]> for Vec3
{
    fn from(value: &[I]) -> Self {
        Vec3 { x: value[0], y: value[1], z: value[2] }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Vec3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z };
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Vec3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z };
    }
}

impl Mul<I> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: I) -> Self::Output {
        Vec3 { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

impl Mul<Vec3> for I {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 { x: self * rhs.x, y: self * rhs.y, z: self * rhs.z }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: (self.x as f64 * rhs).round() as I,
            y: (self.y as f64 * rhs).round() as I,
            z: (self.z as f64 * rhs).round() as I,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: (self * rhs.x as f64).round() as I,
            y: (self * rhs.y as f64).round() as I,
            z: (self * rhs.z as f64).round() as I,
        }
    }
}

impl Div<I> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: I) -> Self::Output {
        Vec3 { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
    }
}

impl Div<Vec3> for I {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3 { x: self / rhs.x, y: self / rhs.y, z: self / rhs.z }
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3 { x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z }
    }
}