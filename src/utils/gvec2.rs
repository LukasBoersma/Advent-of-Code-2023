use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, Div};

use num::{Signed, CheckedAdd};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Vec2<I>(pub I, pub I) where I: num::Integer;

impl<I> Vec2<I>
where I: num::Integer, i32: Into<I>, I: CheckedAdd, I: Copy {
    pub fn x(&self) -> I {
        self.0
    }
    pub fn y(&self) -> I {
        self.1
    }

    pub fn new(x: I, y: I) -> Self {
        Vec2(x, y)
    }

    pub fn zero() -> Self {
        Vec2(0.into(), 0.into())
    }

    pub fn one() -> Self {
        Vec2(1.into(), 1.into())
    }

    pub fn min(&self, other: Self) -> Self {
        Vec2(self.0.min(other.0), self.1.min(other.1))
    }

    pub fn max(&self, other: Self) -> Self {
        Vec2(self.0.max(other.0), self.1.max(other.1))
    }

    pub fn dot(&self, other: Self) -> I {
        self.0 * other.0 + self.1 * other.1
    }

    pub fn cross(&self, other: Self) -> I {
        self.0 * other.1 - self.1 * other.0
    }

    // pub fn cross_i128(&self, other: Self) -> i128 {
    //     self.0 as i128 * other.1 as i128 - self.1 as i128 * other.0 as i128
    // }

    

    pub fn add_checked(&self, other: Self) -> Option<Self> {
        self.0.checked_add(&other.0).and_then(|x| self.1.checked_add(&other.1).map(|y| Vec2(x, y)))
    }

}

impl<I> Vec2<I>
where I: num::Integer, I: Signed {
    pub fn abs(&self) -> Self {
        //let x = num::integer::Integer::abs(self.0);
        //let y = num::integer::Integer::abs(self.1);
        Vec2(self.0.abs(), self.1.abs())
    }

    pub fn manhattan(&self) -> I {
        self.0.abs() + self.1.abs()
    }
}


impl<I> Vec2<I>
where I: num::Integer, I: Into<f64>, f64: Into<I>, I: Copy {
    pub fn length(&self) -> f64 {
        ((self.0 * self.0 + self.1 * self.1).into() as f64).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let length = self.length();
        Vec2((self.0.into() as f64 / length).into() as I, (self.1.into() as f64 / length).into() as I)
    }
}


impl<I> Add<Vec2<I>> for Vec2<I>
where
    I: num::Integer,
    I: std::ops::Add<Output = I>,
{
    type Output = Vec2<I>;

    fn add(self, rhs: Vec2<I>) -> Self::Output {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<I> AddAssign<Vec2<I>> for Vec2<I>
where
    I: num::Integer,
    I: std::ops::AddAssign,
{
    fn add_assign(&mut self, rhs: Vec2<I>) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl<I> Sub<Vec2<I>> for Vec2<I>
where
    I: num::Integer,
    I: std::ops::Sub<Output = I>,
{
    type Output = Vec2<I>;

    fn sub(self, rhs: Vec2<I>) -> Self::Output {
        Vec2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<I> SubAssign for Vec2<I>
where I: num::Integer, I: Copy
{
    fn sub_assign(&mut self, rhs: Self) {
        *self = Vec2(self.0 - rhs.0, self.1 - rhs.1);
    }
}

impl<I> Mul<I> for Vec2<I>
where I: num::Integer, I: Copy
{
    type Output = Self;

    fn mul(self, rhs: I) -> Self::Output {
        Vec2(self.0 * rhs, self.1 * rhs)
    }
}

macro_rules! vec2_integer_impl {
    ($int_type:ty) => {
        impl<I> Mul<Vec2<I>> for $int_type
        where I: num::Integer, $int_type: Into<I>, $int_type: Mul<I, Output = I>
        {
            type Output = Vec2<I>;

            fn mul(self, rhs: Vec2<I>) -> Self::Output {
                Vec2(self.into() as I * rhs.0, self.into() as I * rhs.1)
            }
        }

        impl<I> Div<Vec2<I>> for $int_type
        where I: num::Integer, $int_type: Into<I>, $int_type: Div<I, Output = I>
        {
            type Output = Vec2<I>;
        
            fn div(self, rhs: Vec2<I>) -> Self::Output {
                Vec2::<I>(self / rhs.0, self / rhs.1)
            }
        }
    };
}

vec2_integer_impl!(usize);
vec2_integer_impl!(i32);
vec2_integer_impl!(i64);
vec2_integer_impl!(i128);

impl<I> Div<I> for Vec2<I>
where I: num::Integer, I: Copy
{
    type Output = Self;

    fn div(self, rhs: I) -> Self::Output {
        Vec2(self.0 / rhs, self.1 / rhs)
    }
}

impl<I> Div<Vec2<I>> for Vec2<I>
where I: num::Integer
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Vec2(self.0 / rhs.0, self.1 / rhs.1)
    }
}

impl<I> From<(I,I)> for Vec2<I>
where I: num::Integer
{
    fn from(value: (I, I)) -> Self {
        Self(value.0, value.1)
    }
}
