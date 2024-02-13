#![allow(dead_code)]

use std::f32::consts::PI;

use crate::physics::grid;
use eframe::epaint::{Color32, ColorImage};

const OFFEST: f32 = PI / 3.0;

pub fn get_color(x: f32) -> Color32 {
    let r = ((x + OFFEST * 0.0).sin().powi(2) * 255.0) as u8;
    let g = ((x + OFFEST * 1.0).sin().powi(2) * 255.0) as u8;
    let b = ((x + OFFEST * 2.0).sin().powi(2) * 255.0) as u8;

    Color32::from_rgb(r, g, b)
}

pub fn map_colors(img: &ColorImage, grid: &grid::Grid, max_size: usize) -> Box<[Color32]> {
    let mut colors = vec![Color32::BLACK; max_size];
    for (i, &color) in img.pixels.iter().enumerate() {
        grid.cells[i].iter().for_each(|&j| {
            colors[j] = color;
        });
    }
    colors.into_boxed_slice()
}
