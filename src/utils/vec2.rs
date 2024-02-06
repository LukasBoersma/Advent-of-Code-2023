use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, Div};

type I = i64;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Vec2(pub I, pub I);

impl Vec2 {
    pub fn x(&self) -> I {
        self.0
    }
    pub fn y(&self) -> I {
        self.1
    }
}

impl Vec2 {
    pub fn new(x: I, y: I) -> Self {
        Vec2(x, y)
    }

    pub fn zero() -> Self {
        Vec2(0, 0)
    }

    pub fn one() -> Self {
        Vec2(1, 1)
    }

    pub fn min(&self, other: Self) -> Self {
        Vec2(self.0.min(other.0), self.1.min(other.1))
    }

    pub fn max(&self, other: Self) -> Self {
        Vec2(self.0.max(other.0), self.1.max(other.1))
    }

    pub fn abs(&self) -> Self {
        Vec2(self.0.abs(), self.1.abs())
    }

    pub fn dot(&self, other: Self) -> I {
        self.0 * other.0 + self.1 * other.1
    }

    pub fn cross(&self, other: Self) -> I {
        self.0 * other.1 - self.1 * other.0
    }

    pub fn cross_i128(&self, other: Self) -> i128 {
        self.0 as i128 * other.1 as i128 - self.1 as i128 * other.0 as i128
    }

    pub fn length(&self) -> f64 {
        ((self.0 * self.0 + self.1 * self.1) as f64).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let length = self.length();
        Vec2((self.0 as f64 / length) as I, (self.1 as f64 / length) as I)
    }

    pub fn manhattan(&self) -> I {
        self.0.abs() + self.1.abs()
    }

    pub fn add_checked(&self, other: Self) -> Option<Self> {
        self.0.checked_add(other.0).and_then(|x| self.1.checked_add(other.1).map(|y| Vec2(x, y)))
    }

}

impl Add for Vec2
{
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Vec2
{
    fn add_assign(&mut self, rhs: Self) {
        *self = Vec2(self.0 + rhs.0, self.1 + rhs.1);
    }
}

impl Sub for Vec2
{
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl SubAssign for Vec2
{
    fn sub_assign(&mut self, rhs: Self) {
        *self = Vec2(self.0 - rhs.0, self.1 - rhs.1);
    }
}

impl Mul<I> for Vec2
{
    type Output = Vec2;

    fn mul(self, rhs: I) -> Self::Output {
        Vec2(self.0 * rhs, self.1 * rhs)
    }
}

impl Mul<Vec2> for I
{
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2(self * rhs.0, self * rhs.1)
    }
}

impl Div<I> for Vec2
{
    type Output = Vec2;

    fn div(self, rhs: I) -> Self::Output {
        Vec2(self.0 / rhs, self.1 / rhs)
    }
}

impl Div<Vec2> for I
{
    type Output = Vec2;

    fn div(self, rhs: Vec2) -> Self::Output {
        Vec2(self / rhs.0, self / rhs.1)
    }
}

impl Div<Vec2> for Vec2
{
    type Output = Vec2;

    fn div(self, rhs: Vec2) -> Self::Output {
        Vec2(self.0 / rhs.0, self.1 / rhs.1)
    }
}

impl From<(I,I)> for Vec2
{
    fn from(value: (I, I)) -> Self {
        Vec2(value.0, value.1)
    }
}

impl From<(i32,i32)> for Vec2
{
    fn from(value: (i32, i32)) -> Self {
        Vec2(value.0 as I, value.1 as I)
    }
}