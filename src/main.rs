use std::collections::HashSet;

use image::{Rgb, RgbImage};
use rand::{prelude::SliceRandom, Rng};

const SIZE: usize = 12;
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn neighbour(&self, dx: i32, dy: i32) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct Cell {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
}
fn main() {
    let mut rng = rand::thread_rng();
    let mut image = RgbImage::new(SIZE as u32 * 2 + 1, SIZE as u32 * 2 + 1);

    let mut visited = HashSet::<Position>::new();
    let mut stack = Vec::<Position>::new();
    let mut grid = [Cell::default(); SIZE * SIZE];

    //initial cell
    let ix = rng.gen_range(0..SIZE);
    let iy = rng.gen_range(0..SIZE);
    let ic = Position {
        x: ix as i32,
        y: iy as i32,
    };
    visited.insert(ic);
    stack.push(ic);

    while !stack.is_empty() {
        let curr = match stack.pop() {
            Some(c) => c,
            None => break,
        };
        let up = curr.neighbour(0, -1);
        let down = curr.neighbour(0, 1);
        let right = curr.neighbour(1, 0);
        let left = curr.neighbour(-1, 0);

        let v_up = !visited.contains(&up);
        let v_down = !visited.contains(&down);
        let v_right = !visited.contains(&right);
        let v_left = !visited.contains(&left);
        let mut unvisited = [(up, v_up), (down, v_down), (right, v_right), (left, v_left)]
            .into_iter()
            .filter(|v| v.1)
            .map(|v| v.0)
            .collect::<Vec<_>>();

        if !unvisited.is_empty() {
            unvisited.shuffle(&mut rng);
            stack.push(curr);

            let chosen = unvisited.pop().unwrap();
            if curr.x >= 0
                && curr.x < SIZE as i32
                && curr.y >= 0
                && curr.y < SIZE as i32
                && chosen.x >= 0
                && chosen.x < SIZE as i32
                && chosen.y >= 0
                && chosen.y < SIZE as i32
            {
                let dx = chosen.x - curr.x;
                let dy = chosen.y - curr.y;
                if dx < 0 {
                    grid[curr.x as usize + curr.y as usize * SIZE].left = true;
                    grid[chosen.x as usize + chosen.y as usize * SIZE].right = true;
                }
                if dx > 0 {
                    grid[curr.x as usize + curr.y as usize * SIZE].right = true;
                    grid[chosen.x as usize + chosen.y as usize * SIZE].left = true;
                }
                if dy < 0 {
                    grid[curr.x as usize + curr.y as usize * SIZE].up = true;
                    grid[chosen.x as usize + chosen.y as usize * SIZE].down = true;
                }
                if dy > 0 {
                    grid[curr.x as usize + curr.y as usize * SIZE].down = true;
                    grid[chosen.x as usize + chosen.y as usize * SIZE].up = true;
                }
                stack.push(chosen);
            }
            visited.insert(chosen);
        }
    }

    for y in 0..SIZE as u32 {
        for x in 0..SIZE as u32 {
            let cell = grid[x as usize + y as usize * SIZE];

            image.put_pixel(x * 2 + 1, y * 2 + 1, Rgb::from([255u8, 255u8, 255u8]));

            if cell.left {
                image.put_pixel(x * 2, y * 2 + 1, Rgb::from([255u8, 255u8, 255u8]));
            }
            if cell.right {
                image.put_pixel(x * 2 + 2, y * 2 + 1, Rgb::from([255u8, 255u8, 255u8]));
            }
            if cell.up {
                image.put_pixel(x * 2 + 1, y * 2, Rgb::from([255u8, 255u8, 255u8]));
            }
            if cell.down {
                image.put_pixel(x * 2 + 1, y * 2 + 2, Rgb::from([255u8, 255u8, 255u8]));
            }
        }
    }
    image.put_pixel(0, 1, Rgb::from([255u8, 255u8, 255u8]));
    image.put_pixel(
        SIZE as u32 * 2,
        SIZE as u32 * 2 - 1,
        Rgb::from([255u8, 255u8, 255u8]),
    );

    image
        .save_with_format("maze.png", image::ImageFormat::Png)
        .unwrap();
}
