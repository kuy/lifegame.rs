extern crate rand;
extern crate ncurses;

use self::rand::{thread_rng, Rng};
use self::ncurses::{printw, erase, refresh};
use renderer::Renderer;
use life::Life;
use life::render as render_life;

pub struct World {
    tick: u32,
    data: Vec<Vec<Life>>,
}

pub fn new(n: usize) -> World {
    let mut rng = thread_rng();
    let mut data = Vec::with_capacity(n);
    for _ in 0..n {
        let mut row = Vec::with_capacity(n);
        for _ in 0..n {
            let s = if rng.gen() {
                Life::Live
            } else {
                Life::Dead
            };
            row.push(s);
        }
        data.push(row);
    }

    World {
        tick: 0,
        data: data,
    }
}

pub fn update(world: &World) -> World {
    let n = world.data.len() as i32;
    let mut data: Vec<Vec<Life>> = Vec::with_capacity(n as usize);
    for row in 0..n {
        let mut new_row: Vec<Life> = Vec::with_capacity(n as usize);
        for col in 0..n {
            let num = [
                cell(&world, row - 1, col - 1), cell(&world, row - 1, col), cell(&world, row - 1, col + 1),
                cell(&world, row,     col - 1),                             cell(&world, row,     col + 1),
                cell(&world, row + 1, col - 1), cell(&world, row + 1, col), cell(&world, row + 1, col + 1),
            ].iter().fold(0, |acc, life| {
                match life {
                    Some(Life::Live) => acc + 1,
                    _ => acc,
                }
            });
            let state = match cell(&world, row, col) {
                Some(Life::Live) => match num {
                    3 => Life::Live,
                    2 => Life::Live,
                    _ => Life::Dead,
                },
                Some(Life::Dead) => match num {
                    3 => Life::Live,
                    _ => Life::Dead,
                },
                None => Life::Dead, // something wrong
            };
            new_row.push(state);
        }
        data.push(new_row);
    }

    World {
        tick: world.tick + 1,
        data: data,
    }
}

pub fn cell(world: &World, row: i32, col: i32) -> Option<&Life> {
    let n = world.data.len() as i32;
    if row < 0 || n - 1 < row || col < 0 || n - 1 < col {
        None
    } else {
        match world.data.get(row as usize) {
            Some(r) => r.get(col as usize),
            None => None,
        }
    }
}

pub fn done(world: &World) -> bool {
    world.tick > 100
}

impl Renderer for World {
    fn render(&self) {
        erase();
        let status = format!("[{}] render\n", self.tick);
        printw(&status);
        for row in self.data.iter() {
            for cell in row.iter() {
                let c = render_life(&cell);
                printw(&c);
            }
            printw("\n");
        }
        refresh();
    }
}
