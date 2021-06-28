use std::ops::Add;

use super::Vec2;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rect<T> {
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}

impl<T> Rect<T>
where
    T: Add<Output = T>,
    T: Copy,
    T: Ord,
{
    pub fn rect(x: T, y: T, width: T, height: T) -> Self {
        Rect {
            x,
            y,
            width,
            height,
        }
    }

    pub fn square(x: T, y: T, size: T) -> Self {
        Self::rect(x, y, size, size)
    }

    pub fn left(&self) -> T {
        self.x
    }

    pub fn right(&self) -> T {
        self.x + self.width
    }

    pub fn top(&self) -> T {
        self.y + self.height
    }

    pub fn bottom(&self) -> T {
        self.y
    }

    pub fn pos(&self) -> (T, T) {
        (self.x, self.y)
    }

    pub fn size(&self) -> (T, T) {
        (self.width, self.height)
    }

    pub fn inside(&self, point: Vec2<T>) -> bool {
        point.x >= self.left()
            && point.y >= self.bottom()
            && point.x <= self.right()
            && point.y <= self.top()
    }
}
