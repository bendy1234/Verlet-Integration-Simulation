use eframe::{
    emath::{Pos2, Vec2},
    epaint::Color32,
};

pub struct Object {
    pub(super) pos: Pos2,
    pub(super) last_pos: Pos2,
    pub(super) acceleration: Vec2,
    pub color: Color32,
}

impl Object {
    pub fn new(pos: Pos2) -> Self {
        Self {
            pos,
            last_pos: pos,
            acceleration: Vec2::ZERO,
            color: Color32::TRANSPARENT,
        }
    }

    pub(super) fn tick(&mut self, delta: f32) {
        let pos = self.pos;
        let displacement = pos - self.last_pos;
        let new_pos =
            pos + displacement + (self.acceleration - displacement * 00.0) * delta.powi(2);

        self.last_pos = pos;
        self.pos = new_pos;

        self.acceleration = Vec2::ZERO;
    }

    #[inline]
    pub fn get_pos(&self) -> Pos2 {
        self.pos
    }
}
