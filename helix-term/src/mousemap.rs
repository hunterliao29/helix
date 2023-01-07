use std::collections::HashMap;

use helix_view::{graphics::Rect, input::MouseEventKind};

use crate::keymap::MappableCommand;
pub struct Mousemap {
    area: Rect,
    pub logic: Box<dyn Fn(u16, u16) -> Option<MappableCommand>>,
}

impl Mousemap {
    pub fn new(area: Rect, logic: Box<dyn Fn(u16, u16) -> Option<MappableCommand>>) -> Self {
        Self { area, logic }
    }
    pub fn in_area(&self, row: u16, column: u16) -> bool {
        self.area.x <= column
            && self.area.x + self.area.width > column
            && self.area.y <= row
            && self.area.y + self.area.height > row
    }
}

pub struct Mousemaps {
    pub map: HashMap<MouseEventKind, Vec<Mousemap>>,
}

impl Mousemaps {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    pub fn add_mouse_map(&mut self, kind: MouseEventKind, map: Mousemap) {
        match self.map.get_mut(&kind) {
            Some(arr) => arr.push(map),
            None => {
                self.map.insert(kind, vec![map]);
            }
        }
    }
    pub fn has_logic(&self, kind: MouseEventKind, row: u16, column: u16) -> Option<&Mousemap> {
        match self.map.get(&kind) {
            Some(click_logics) => click_logics.iter().rev().find(|x| x.in_area(row, column)),
            None => None,
        }
    }
    pub fn clear(&mut self) {
        self.map.clear();
    }
}

impl Default for Mousemaps {
    fn default() -> Self {
        Self::new()
    }
}
