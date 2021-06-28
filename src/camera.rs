use crate::util::Vec2;

pub struct Camera {
    pub position: Vec2<i32>,
    pub width: u32,
    pub height: u32,
}

impl Camera {
    pub fn world_to_view(&self, pos: &Vec2<i32>) -> Vec2<i32> {
        Vec2::new(pos.x - pos.y, (pos.x + pos.y) / 2)
    }

    pub fn view_to_world(&self, pos: &Vec2<i32>) -> Vec2<i32> {
        Vec2::new((2 * pos.y + pos.x) / 2, (2 * pos.y - pos.x) / 2)
    }
}
