#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry; //这个和《The embedonomicon》中的entry宏原理是一样的，原理是库函数才是真正的入口，然后库函数再来加载用户函数
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, Timer},
};
use panic_halt as _;

#[entry]
fn main() -> ! {
    //发散函数，《The embedonomicon》也说过

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let circle2 = [
        // 外圈全亮
        [1, 1, 1, 1, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 1, 1, 1, 1],
    ];
    let circle1 = [
        // 中间圈全亮
        [0, 0, 0, 0, 0],
        [0, 1, 1, 1, 0],
        [0, 1, 0, 1, 0],
        [0, 1, 1, 1, 0],
        [0, 0, 0, 0, 0],
    ];
    let circle0 = [
        // 里圈全亮
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    let light_up_duration = 120;

    loop {
        display.show(&mut timer, circle0, light_up_duration);
        display.clear();
        display.show(&mut timer, circle1, light_up_duration);
        display.clear();
        display.show(&mut timer, circle2, light_up_duration);
        display.clear();
        timer.delay_ms(500u16);

    }
}
