use eframe::emath::{Pos2, Vec2};

use rayon::prelude::*;

pub struct Cell {
    objects: [usize; 4],
    len: u8,
}

impl Cell {
    pub const EMPTY: Self = Self::new();

    const fn new() -> Self {
        Self {
            objects: [0; 4],
            len: 0,
        }
    }

    fn add(&mut self, index: usize) {
        if (self.len as usize) < 4 {
            unsafe {
                *self.objects.get_unchecked_mut(self.len as usize) = index;
            };
            self.len += 1;
        }
    }

    #[inline]
    fn clear(&mut self) {
        self.len = 0;
    }

    pub fn iter(&self) -> std::slice::Iter<'_, usize> {
        self.objects[0..self.len as usize].iter()
    }
}

pub struct Grid {
    pub(crate) size: (i16, i16),
    pub cells: Box<[Cell]>,
}

impl Grid {
    pub fn new(size: Vec2) -> Self {
        if size.max_elem() > i16::MAX.into() {
            panic!("size too large");
        }

        let size = (size.x as i16, size.y as i16);
        let area = size.0 as usize * size.1 as usize;

        let mut cells = Vec::with_capacity(area);
        for _ in 0..area {
            cells.push(Cell::new());
        }
        let cells = cells.into_boxed_slice();

        Self { size, cells }
    }

    #[inline]
    fn get_index(&self, x: i16, y: i16) -> usize {
        y as usize * self.size.0 as usize + x as usize
    }

    pub fn add_index(&mut self, index: usize, pos: Pos2) {
        unsafe {
            self.cells
                .get_unchecked_mut(self.get_index(pos.x as i16, pos.y as i16))
                .add(index)
        };
    }

    pub fn clear(&mut self) {
        self.cells.par_iter_mut().for_each(Cell::clear);
    }

    fn get_cell(&self, x: i16, y: i16) -> &Cell {
        if x < 0 || y < 0 || x >= self.size.0 || y >= self.size.1 {
            return &Cell::EMPTY;
        }

        unsafe { self.cells.get_unchecked(self.get_index(x, y)) }
    }

    pub fn get_nearby_cells(&self, index: usize) -> [&Cell; 9] {
        let (x, y) = (
            (index % self.size.0 as usize) as i16,
            (index / self.size.0 as usize) as i16,
        );
        [
            self.get_cell(x - 1, y - 1),
            self.get_cell(x, y - 1),
            self.get_cell(x + 1, y - 1),
            self.get_cell(x - 1, y),
            self.get_cell(x, y),
            self.get_cell(x + 1, y),
            self.get_cell(x - 1, y + 1),
            self.get_cell(x, y + 1),
            self.get_cell(x + 1, y + 1),
        ]
    }
}
