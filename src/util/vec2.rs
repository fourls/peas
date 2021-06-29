use cgmath::{num_traits::Num, AbsDiffEq};
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Default, PartialEq, Eq, Clone, Copy, Hash, Debug, Serialize, Deserialize)]
pub struct Vec2<T>
where
    T: Num + Copy,
{
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T>
where
    T: Num + Copy,
{
    pub fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }

    pub fn sqr_magnitude(&self) -> T {
        self.x * self.x + self.y * self.y
    }
}

impl<T> Vec2<T>
where
    T: Num + Copy,
    T: Into<f32>,
    T: From<f32>,
{
    pub fn magnitude(&self) -> f32 {
        f32::sqrt(self.sqr_magnitude().into())
    }

    pub fn normalize(&mut self) {
        let mag = self.magnitude();

        if !mag.abs_diff_eq(&0.0, f32::EPSILON) {
            self.x = (self.x.into() / mag).into();
            self.y = (self.y.into() / mag).into();
        }
    }

    pub fn distance(&self, other: Self) -> f32 {
        (other - *self).magnitude()
    }
}

impl<T> From<(T, T)> for Vec2<T>
where
    T: Num + Copy,
{
    fn from(obj: (T, T)) -> Self {
        Self { x: obj.0, y: obj.1 }
    }
}

impl<T> Mul<T> for Vec2<T>
where
    T: Num + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> Add for Vec2<T>
where
    T: Num + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Sub for Vec2<T>
where
    T: Num + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Div<T> for Vec2<T>
where
    T: Num + Copy,
{
    type Output = Vec2<T>;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T> AddAssign for Vec2<T>
where
    T: Num + Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<T> SubAssign for Vec2<T>
where
    T: Num + Copy,
{
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<T> MulAssign<T> for Vec2<T>
where
    T: Num + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}

impl<T> DivAssign<T> for Vec2<T>
where
    T: Num + Copy,
{
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs;
    }
}
