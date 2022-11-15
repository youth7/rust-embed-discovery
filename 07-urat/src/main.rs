#![no_main]
#![no_std]

use cortex_m_rt::entry; // cortex-m的运行时
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

use microbit::{
    hal::prelude::*,
    hal::uarte::{Baudrate, Parity, Uarte},
};

static mut TX_BUF: [u8; 1] = [0; 1];
static mut RX_BUF: [u8; 1] = [0; 1];

#[entry]
fn main() -> ! {
    rtt_init_print!(); //初始化rtt
    let board = microbit::Board::take().unwrap();
    let uarte_instance = Uarte::new(//构造一个uarte实例
        board.UARTE0,
        board.uart.into(),
        Parity::EXCLUDED,
        Baudrate::BAUD115200,
    );
    //将uarte实例上的tx和rx提取出来单独使用
    let (mut tx, rx) = uarte_instance
        .split(unsafe { &mut TX_BUF }, unsafe { &mut RX_BUF })
        .unwrap();
    nb::block!(tx.write(b'X')).unwrap(); //往串口上写入字符X
    nb::block!(tx.flush()).unwrap(); //强制清空缓冲区，立即写入

    loop {}
}