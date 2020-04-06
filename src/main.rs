use crate::helpers::*;

mod helpers;

use minifb::{Key, Window, WindowOptions};
// use rand::seq::SliceRandom;
use rand::seq::SliceRandom;
use rand::Rng;
use rayon::prelude::*;

const WIDTH: usize = 800;
const HEIGHT: usize = 800;
const GRID_CELL_SIZE: usize = 4;
const BOTTOM_EXTRA: usize = 200;

fn main() {
    let mut output_buffer: Vec<u32> = vec![0; WIDTH * (HEIGHT + BOTTOM_EXTRA)];
    let mut grid_buffer: Vec<Cell> =
        vec![Cell::new(false); (WIDTH / GRID_CELL_SIZE) * (HEIGHT / GRID_CELL_SIZE)];
    let mut render_buffer: Vec<Col> =
        vec![Col::new(0.0, 0.0, 0.0); WIDTH * (HEIGHT + BOTTOM_EXTRA)];
    let mut window = Window::new("", WIDTH, HEIGHT + BOTTOM_EXTRA, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    let mut size_log = vec![];
    let mut change_log = vec![];
    let mut smooth_change_log = vec![];
    let mut max_size = 0;
    let mut size_log_len = 0;
    let mut max_change = 0;

    // Initialize the grid
    grid_buffer
        .par_iter_mut()
        .enumerate()
        .for_each(|(_, grid_cell)| {
            let is_alive = if rand::thread_rng().gen_range(0.0, 1.0) < 0.001 {
                true
            } else {
                false
            };

            let cell = Cell::new(is_alive);

            // Update render buffer with result
            *grid_cell = cell;
        });

    // Main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut rng = rand::thread_rng();

        // let time_since_update = time_since_update(&mut viewport.time);
        // if time_since_update * 1000.0 < 1000.0 {
        //     thread::sleep(time::Duration::from_millis(100));
        //     continue;
        // } else {
        //     app::update_time(&mut viewport.time);
        // }

        let mut new_cells: Vec<Cell> =
            vec![Cell::new(false); (WIDTH / GRID_CELL_SIZE) * (HEIGHT / GRID_CELL_SIZE)];
        // let mut new_cells: Vec<Cell> = grid_buffer.clone();
        // let mut grid_buffer_clone: Vec<Cell> = grid_buffer.clone();

        let mut index = Index::new(0, (WIDTH / GRID_CELL_SIZE) * (HEIGHT / GRID_CELL_SIZE));
        // for i in 0..grid_buffer.len() {
        for (i, new_cell) in new_cells.iter_mut().enumerate() {
            // let mut new_cell = new_cells[i];
            index.assign(i);
            // let cell = grid_buffer[i];

            let above = index - (WIDTH / GRID_CELL_SIZE);
            let below = index + (WIDTH / GRID_CELL_SIZE);

            let adjacent_indices = vec![index - 1, index + 1, above, below];

            let neighbors = adjacent_indices
                .iter()
                .map(|j| grid_buffer[usize::from(*j)])
                .collect::<Vec<_>>();

            let num_neighbors =
                neighbors.iter().fold(
                    0,
                    |acc, neighbor| if neighbor.is_alive { acc + 1 } else { acc },
                );

            if num_neighbors == 1 {
                new_cell.is_alive = true;
            }
        }

        let mut indices: Vec<usize> = (0..new_cells.len()).collect();
        indices.shuffle(&mut rng);

        for i in &indices {
            index.assign(*i);

            let above = index - (WIDTH / GRID_CELL_SIZE);
            let above_above = index - 2 * (WIDTH / GRID_CELL_SIZE);
            let below = index + (WIDTH / GRID_CELL_SIZE);
            let below_below = index + 2 * (WIDTH / GRID_CELL_SIZE);

            let neighbor_indices = vec![
                above - 2,
                above - 1,
                above,
                above + 1,
                above + 2,
                index - 2,
                index - 1,
                index + 1,
                index + 2,
                below - 2,
                below - 1,
                below,
                below + 1,
                below + 2,
                above_above - 2,
                above_above - 1,
                above_above,
                above_above + 1,
                above_above + 2,
                below_below - 2,
                below_below - 1,
                below_below,
                below_below + 1,
                below_below + 2,
            ];
            // let neighbor_indices = vec![
            //     above - 1,
            //     above,
            //     above + 1,
            //     index + 1,
            //     below + 1,
            //     below,
            //     below - 1,
            //     index - 1,
            // ];

            let neighbors = neighbor_indices
                .iter()
                .map(|j| new_cells[usize::from(*j)])
                .collect::<Vec<_>>();

            let num_neighbors =
                neighbors.iter().fold(
                    0,
                    |acc, neighbor| if neighbor.is_alive { acc + 1 } else { acc },
                );

            if num_neighbors > 0 {
                new_cells[usize::from(index)].is_alive = false;
            }
        }

        for (cell, new_cell) in grid_buffer.iter_mut().zip(new_cells.iter()) {
            if new_cell.is_alive {
                cell.is_alive = true;
            }
        }

        let mut size = 0;

        for i in indices {
            index.assign(i);

            let above = index - (WIDTH / GRID_CELL_SIZE);
            let above_above = index - 2 * (WIDTH / GRID_CELL_SIZE);
            let below = index + (WIDTH / GRID_CELL_SIZE);
            let below_below = index + 2 * (WIDTH / GRID_CELL_SIZE);

            let adjacent_indices = vec![index - 1, index + 1, above, below];

            let diagonal_indices = vec![
                above - 2,
                above - 1,
                above + 1,
                above + 2,
                index - 2,
                index + 2,
                below - 2,
                below - 1,
                below + 1,
                below + 2,
                above_above - 2,
                above_above - 1,
                above_above,
                above_above + 1,
                above_above + 2,
                below_below - 2,
                below_below - 1,
                below_below,
                below_below + 1,
                below_below + 2,
            ];

            let adjacent_neighbors = adjacent_indices
                .iter()
                .map(|j| grid_buffer[usize::from(*j)])
                .collect::<Vec<_>>();

            let diagonal_neighbors = diagonal_indices
                .iter()
                .map(|j| new_cells[usize::from(*j)])
                .collect::<Vec<_>>();

            let num_adjacent_neighbors =
                adjacent_neighbors.iter().fold(
                    0,
                    |acc, neighbor| if neighbor.is_alive { acc + 1 } else { acc },
                );

            let num_diagonal_neighbors =
                diagonal_neighbors.iter().fold(
                    0,
                    |acc, neighbor| if neighbor.is_alive { acc + 1 } else { acc },
                );

            if num_diagonal_neighbors > 0 && num_adjacent_neighbors < 2 {
                grid_buffer[usize::from(index)].is_alive = false;
            }

            grid_buffer[usize::from(index)].update_age();
            if grid_buffer[usize::from(index)].is_alive {
                size += 1;
            }
        }

        size_log.push(size);
        size_log_len += 1;
        if size > max_size {
            max_size = size;
        }
        let change = if size_log_len > 1 {
            size - size_log[size_log_len - 2]
        } else {
            0
        };
        change_log.push(change);
        if change > max_change {
            max_change = change;
        }
        let d = 20;
        let d: i32 = if size_log_len as i32 - d <= 0 {
            size_log_len as i32
        } else {
            d
        };

        let smooth_change = (0..d)
            .collect::<Vec<i32>>()
            .iter()
            .enumerate()
            .map(|(i, _)| change_log[size_log_len - i - 1])
            .collect::<Vec<i32>>()
            .iter()
            .sum::<i32>()
            / d;

        smooth_change_log.push(smooth_change);

        render(
            &mut window,
            &mut render_buffer,
            &mut output_buffer,
            // &mut temp_grid_buffer,
            &mut grid_buffer,
            &size_log,
            max_size,
            size_log_len,
            &change_log,
            max_change,
            &smooth_change_log,
        );
    }
}

fn render(
    window: &mut Window,
    render_buffer: &mut Vec<Col>,
    output_buffer: &mut Vec<u32>,
    grid_buffer: &mut Vec<Cell>,
    size_log: &Vec<i32>,
    max_size: i32,
    size_log_len: usize,
    change_log: &Vec<i32>,
    max_change: i32,
    smooth_change_log: &Vec<i32>,
) {
    // Iterate pixels
    render_buffer
        .par_iter_mut()
        .enumerate()
        .for_each(|(i, pixel)| {
            let uv = uv(i, WIDTH as f32, (HEIGHT + BOTTOM_EXTRA) as f32);
            let pixel_coordinates =
                uv_to_pixel_coordinates(uv, WIDTH as f32, (HEIGHT + BOTTOM_EXTRA) as f32);

            let bottom_pixel = pixel_coordinates.y >= HEIGHT;
            if bottom_pixel {
                *pixel = Col::black();
                return;
            }

            let grid_index =
                index_to_grid_index(i, WIDTH as f32, HEIGHT as f32, GRID_CELL_SIZE as f32);

            let grid_cell = grid_buffer[grid_index];
            let col = grid_cell.into();

            // Update render buffer with result
            *pixel = col;
        });

    let len = size_log_len;
    for (i, entry) in size_log.iter().enumerate() {
        let x = (i as f32 * (WIDTH as f32 / len as f32)) as usize;
        let y = HEIGHT + BOTTOM_EXTRA
            - (*entry as f32 * (BOTTOM_EXTRA as f32 / max_size as f32)) as usize;
        let index = Index::new(
            pixel_coordinates_to_index(x, y, WIDTH),
            WIDTH * (HEIGHT + BOTTOM_EXTRA),
        );
        render_buffer[usize::from(index)] = Col::white();

        let change = change_log[i];
        let y = HEIGHT + BOTTOM_EXTRA
            - (change as f32 * (BOTTOM_EXTRA as f32 / max_change as f32)) as usize;
        let index = Index::new(
            pixel_coordinates_to_index(x, y, WIDTH),
            WIDTH * (HEIGHT + BOTTOM_EXTRA),
        );
        render_buffer[usize::from(index)] = Col::new(1.0, 1.0, 1.0);

        let smooth_change = smooth_change_log[i];
        let y = HEIGHT + BOTTOM_EXTRA
            - (smooth_change as f32 * (BOTTOM_EXTRA as f32 / max_change as f32)) as usize;
        let index = Index::new(
            pixel_coordinates_to_index(x, y, WIDTH),
            WIDTH * (HEIGHT + BOTTOM_EXTRA),
        );
        render_buffer[usize::from(index)] = Col::yellow();
    }

    // Update frame buffer with render buffer
    for (col_1, col_2) in render_buffer.iter().zip(output_buffer.iter_mut()) {
        *col_2 = col_to_rgb_u32(*col_1);
    }

    // Update window
    window.update_with_buffer(output_buffer).unwrap();
}

#[derive(Clone, Copy)]
pub struct Cell {
    pub is_alive: bool,
    pub age: u8,
    pub debug_col: Col,
}

impl Cell {
    pub fn new(is_alive: bool) -> Cell {
        Cell {
            is_alive,
            age: 0,
            debug_col: Col::black(),
        }
    }

    pub fn update_age(&mut self) {
        if self.is_alive {
            self.age += 1;
        } else {
            self.age = 0;
        }
    }
}

impl Into<f32> for Cell {
    fn into(self) -> f32 {
        if self.is_alive {
            1.0 as f32
        } else {
            0.0 as f32
        }
    }
}

impl Into<Col> for Cell {
    fn into(self) -> Col {
        let age = self.age;
        // let young_col = Col::new(0.4, 1.0, 0.0) * 0.8;
        let young_col = Col::new(0.1, 0.7, 1.0);
        let old_col = Col::yellow() * 0.7;
        let really_old_col = Col::red() * 0.7;
        let is_alive: f32 = self.into();
        let mix =
            1.0 - ((2.0 / (1.0 + 2.71828_f32.powf(-1.0 * age as f32 / 40.0))) - 1.0).powf(2.0);
        let mix_2 =
            1.0 - ((2.0 / (1.0 + 2.71828_f32.powf(-1.0 * age as f32 / 130.0))) - 1.0).powf(2.0);

        // let col = (mix_col(young_col, old_col, mix * 0.5) + mix_col(young_col, old_col, mix * 0.5))
        //     * is_alive;
        return mix_col(mix_col(young_col, old_col, mix), really_old_col, mix_2)
            * is_alive
            * (1.0 - 1.0 / (age as f32 / 2.0 + 1.0));
        // self.debug_col
    }
}
