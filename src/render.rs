use eframe::{
    egui::Ui,
    emath::Vec2,
    epaint::{Color32, Rect, Rounding},
};
use rayon::prelude::*;

use crate::physics::object::Object;

pub const PADDING: Vec2 = Vec2::splat(5.0);

pub fn draw_content(size: Vec2, objects: &[Object], ui: &mut Ui) {
    // calcuate the scale
    let min_size = ui.min_size() - PADDING * 2.0;
    let scale = ((min_size.x / size.x).min(min_size.y / size.y)).max(1.0);
    let start_pos =
        ui.min_rect().left_top() + PADDING + ((min_size - size * scale) / 2.0).max(Vec2::ZERO);

    let painter = ui.painter();

    painter.rect_filled(
        Rect::from_min_size(start_pos - PADDING / 2.0, size * scale + PADDING),
        Rounding::ZERO,
        Color32::BLACK,
    );
    let half_scale = scale / 2.0;

    let chunk_size = (objects.len() / 4).max(10);

    let offset = start_pos + Vec2::splat(half_scale);

    objects.par_chunks(chunk_size).for_each(|chunk| {
        chunk.iter().for_each(|obj| {
            painter.circle_filled(
                offset + obj.get_pos().to_vec2() * scale,
                half_scale,
                obj.color,
            );
        })
    });
}
