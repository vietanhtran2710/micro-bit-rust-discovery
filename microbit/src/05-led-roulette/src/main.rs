#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, Timer},
};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut light_it_all = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];
    let (mut row, mut col) = (0_i32, 0_i32);
    let dir: [[i32; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];
    let mut dir_index = 0;

    loop {
        light_it_all[row as usize][col as usize] = 1;
        display.show(&mut timer, light_it_all, 10);
        let row_outside: bool = row + dir[dir_index][0] < 0 || row + dir[dir_index][0] >= 5;
        let col_outside: bool = col + dir[dir_index][1] < 0 || col + dir[dir_index][1] >= 5;
        if row_outside || col_outside {
            dir_index = (dir_index + 1) % 4;
        }
        light_it_all[row as usize][col as usize] = 0;
        row += dir[dir_index][0];
        col += dir[dir_index][1];
        timer.delay_ms(10_u32);
    }
}