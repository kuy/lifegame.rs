extern crate lifegame;
extern crate ncurses;

use std::{thread, time};
use ncurses::*;
use lifegame::*;
use lifegame::renderer::*;

fn main() {
    let mut world: world::World = world::new(10);
    let dur = time::Duration::from_millis(750);

    initscr();

    loop {
        world.render();

        thread::sleep(dur);

        world = world::update(&world);

        if world::done(&world) {
            break;
        }
    }

    getch();
    endwin();
}
