use std::collections::HashSet;

use crow::glutin::event::{ElementState as KeyState, VirtualKeyCode as Key};

#[derive(Default)]
pub struct Input {
    down: HashSet<Key>,
    just_pressed: HashSet<Key>,
}

impl Input {
    pub fn is_down(&self, key: Key) -> bool {
        self.down.contains(&key)
    }

    pub fn is_just_pressed(&self, key: Key) -> bool {
        self.just_pressed.contains(&key)
    }

    pub fn get_down(&self) -> std::collections::hash_set::Iter<Key> {
        self.down.iter()
    }

    pub fn get_just_pressed(&self) -> std::collections::hash_set::Iter<Key> {
        self.just_pressed.iter()
    }

    pub fn frame_end(&mut self) {
        self.just_pressed.clear()
    }

    pub fn process_event(&mut self, key: Key, state: KeyState) {
        match state {
            KeyState::Pressed => {
                if !self.down.contains(&key) {
                    self.just_pressed.insert(key);
                }

                self.down.insert(key);
            }
            KeyState::Released => {
                self.down.remove(&key);
            }
        }
    }
}
