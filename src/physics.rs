pub mod grid;
pub mod object;

use crate::utill;

use eframe::{
    emath::{Pos2, Vec2},
    epaint::Color32,
};
use rayon::prelude::*;
use std::f32::consts::PI;

const GRAVITY: Vec2 = Vec2::new(0.0, 20.0);

static mut OBJECTS: Vec<object::Object> = Vec::new();
pub struct Solver {
    size: Vec2,
    area: f32,
    max_objects: u32,
    grid: grid::Grid,
    colors: Option<Box<[Color32]>>,
}

impl Solver {
    pub fn new(size: Vec2) -> Self {
        let area = size.x * size.y;
        let grid = grid::Grid::new(size);
        let max_objects = (area * 1.14) as u32;
        unsafe {
            OBJECTS = Vec::with_capacity(max_objects as usize);
        }

        Self {
            size,
            area,
            max_objects,
            grid,
            colors: None,
        }
    }

    pub fn set_colors(&mut self, colors: Option<Box<[Color32]>>) {
        if colors.is_some()
            && colors.clone().unwrap().len()
                >= unsafe { OBJECTS.len() }.max(self.max_objects as usize)
        {
            self.colors = colors;
        } else {
            self.colors = None;
        }
    }

    pub fn has_colors(&self) -> bool {
        self.colors.is_some()
    }

    pub fn objects(&self) -> &Vec<object::Object> {
        unsafe { &OBJECTS }
    }

    pub fn get_grid(&self) -> &grid::Grid {
        &self.grid
    }

    pub fn max_objects(&self) -> u32 {
        self.max_objects
    }

    pub fn reset(&mut self) {
        unsafe { OBJECTS.clear() };
    }

    pub fn set_size(&mut self, size: Vec2) {
        self.size = size;
        self.colors = None;
        self.area = size.x * size.y;
        self.grid = grid::Grid::new(size);
        self.max_objects = (self.area * 1.14) as u32;
        unsafe { OBJECTS = Vec::with_capacity(self.max_objects as usize) };
    }

    pub fn get_size(&self) -> Vec2 {
        self.size
    }

    pub fn tick(&mut self, delta: f32) {
        if unsafe { OBJECTS.len() } < self.max_objects as usize {
            self.add_objects(10);
        }

        let sub_delta = delta / 8.0;
        for _ in 0..8 {
            self.update_grid();
            // do colision checks
            self.solve_collisions();
            // update objects
            self.update_objects(sub_delta);
        }
    }

    fn update_objects(&mut self, sub_delta: f32) {
        unsafe {
            OBJECTS.par_iter_mut().for_each(|object| {
                object.acceleration += GRAVITY;
                object.tick(sub_delta);
                object.pos = object
                    .get_pos()
                    .clamp(Pos2::ZERO, self.size.to_pos2() - Vec2::splat(1.0));
            })
        };
    }

    fn update_grid(&mut self) {
        self.grid.clear();
        unsafe {
            OBJECTS
                .iter()
                .enumerate()
                .for_each(|(i, object)| self.grid.add_index(i, object.get_pos()))
        };
    }

    fn add_objects(&mut self, count: i32) {
        for i in 0..count {
            let index = unsafe { OBJECTS.len() };
            let mut object = object::Object::new(Pos2::new(0.0, 5.0 + 1.1 * i as f32));
            object.pos.x -= 0.2;
            object.color = match &self.colors {
                Some(colors) => colors[index],
                None => utill::get_color(index as f32 / self.max_objects as f32 * PI),
            };
            unsafe { OBJECTS.push(object) };
        }
    }

    fn solve_collisions(&self) {
        let chunk_size = self.size.x as usize * 4;
        let chunks = (0..self.area as usize).into_par_iter().chunks(chunk_size);
        // do 2 passes to prevent data race
        chunks.clone().step_by(2).for_each(|chunk| {
            for cell_index in chunk {
                self.process_cell(cell_index);
            }
        });
        chunks.skip(1).step_by(2).for_each(|chunk| {
            for cell_index in chunk {
                self.process_cell(cell_index);
            }
        });
    }

    fn process_cell(&self, index: usize) {
        let near = self.grid.get_nearby_cells(index);
        let center = near[4];
        for &i in center.iter() {
            let object = &mut unsafe { OBJECTS.get_unchecked_mut(i) };
            near.iter().flat_map(|&cell| cell.iter()).for_each(|&j| {
                Solver::solve_collision(j, object);
            });
        }
    }

    fn solve_collision(j: usize, object: &mut object::Object) {
        let other = &mut unsafe { OBJECTS.get_unchecked_mut(j) };
        let displacement = object.get_pos() - other.get_pos();
        let distance_squared = displacement.length_sq();
        if 0.0001 < distance_squared && distance_squared < 1.0 {
            let distance = distance_squared.sqrt();
            let delta = (1.0 - distance) / 2.0;
            // there should be only one thread trying to access a whole chunk of objects
            let collision_vec = (displacement / distance) * delta;
            object.pos += collision_vec;
            other.pos -= collision_vec;
        }
    }
}
