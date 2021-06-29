use std::collections::HashSet;

use crow::glutin::{dpi::LogicalPosition, event::ElementState as KeyState};

pub use crow::glutin::event::{MouseButton, VirtualKeyCode as Key};

use crate::{constants::VIEW_HEIGHT, util::Vec2};

#[derive(Default, Debug)]
pub struct Input {
    key_down: HashSet<Key>,
    key_just_pressed: HashSet<Key>,
    mouse_down: HashSet<MouseButton>,
    mouse_just_pressed: HashSet<MouseButton>,
    mouse_pos: Vec2<i32>,
}

impl Input {
    pub fn is_down(&self, key: Key) -> bool {
        self.key_down.contains(&key)
    }

    pub fn is_just_pressed(&self, key: Key) -> bool {
        self.key_just_pressed.contains(&key)
    }

    pub fn is_mouse_down(&self, button: MouseButton) -> bool {
        self.mouse_down.contains(&button)
    }

    pub fn is_mouse_just_pressed(&self, button: MouseButton) -> bool {
        self.mouse_just_pressed.contains(&button)
    }

    pub fn get_down(&self) -> std::collections::hash_set::Iter<Key> {
        self.key_down.iter()
    }

    pub fn get_just_pressed(&self) -> std::collections::hash_set::Iter<Key> {
        self.key_just_pressed.iter()
    }

    pub fn mouse_pos(&self) -> Vec2<i32> {
        self.mouse_pos
    }

    pub fn frame_end(&mut self) {
        self.key_just_pressed.clear();
        self.mouse_just_pressed.clear();
    }

    pub fn process_keyboard_input(&mut self, key: Key, state: KeyState) {
        match state {
            KeyState::Pressed => {
                if !self.key_down.contains(&key) {
                    self.key_just_pressed.insert(key);
                }

                self.key_down.insert(key);
            }
            KeyState::Released => {
                self.key_down.remove(&key);
            }
        }
    }

    pub fn process_mouse_input(&mut self, button: MouseButton, state: KeyState) {
        match state {
            KeyState::Pressed => {
                if !self.mouse_just_pressed.contains(&button) {
                    self.mouse_just_pressed.insert(button);
                }

                self.mouse_down.insert(button);
            }
            KeyState::Released => {
                self.mouse_down.remove(&button);
            }
        }
    }

    pub fn process_cursor_moved(&mut self, pos: LogicalPosition<i32>) {
        self.mouse_pos = Vec2::new(pos.x, VIEW_HEIGHT as i32 - pos.y);
    }
}
