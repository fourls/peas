use std::ops::{Add, AddAssign, Mul};

#[derive(Default, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T>
where
    T: Add<Output = T>,
    T: Copy,
{
    pub fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }
}

impl<T> Into<cgmath::Vector2<T>> for Vec2<T> {
    fn into(self) -> cgmath::Vector2<T> {
        cgmath::Vector2::new(self.x, self.y)
    }
}

impl<T> From<(T, T)> for Vec2<T> {
    fn from(obj: (T, T)) -> Self {
        Self { x: obj.0, y: obj.1 }
    }
}

impl<X> Mul<X> for Vec2<X>
where
    X: Copy,
    X: Mul<Output = X>,
    X: Add<Output = X>,
{
    type Output = Vec2<<X as Mul>::Output>;

    fn mul(self, rhs: X) -> Self::Output {
        Vec2::new(self.x * rhs, self.y * rhs)
    }
}

impl<X> Add for Vec2<X>
where
    X: Add<Output = X>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<X> AddAssign for Vec2<X>
where
    X: Add<Output = X>,
    X: Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
