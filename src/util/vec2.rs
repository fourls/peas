use std::ops::Add;
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T>
where
    T: Add<T>,
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
