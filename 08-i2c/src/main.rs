#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;

use microbit::hal::prelude::*;

use microbit::{
    hal::twim::Twim,
    pac::twim0::frequency::FREQUENCY_A,
};

const ACCELEROMETER_ADDR: u8 = 0b0011001;//加速计地址
const MAGNETOMETER_ADDR: u8 = 0b0011110;//磁力仪地址

const ACCELEROMETER_ID_REG: u8 = 0x0f;//加速计的寄存器地址
const MAGNETOMETER_ID_REG: u8 = 0x4f;//磁力仪的寄存器地址

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    let mut i2c =  Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) ;//构造一个twim对象，它兼容i2c

    let mut acc = [0];//缓存，用来存放i2c设备的响应信息
    let mut mag = [0];

    // First write the address + register onto the bus, then read the chip's responses
    i2c.write_read(ACCELEROMETER_ADDR, &[ACCELEROMETER_ID_REG], &mut acc).unwrap();//向设备的寄存器写入信息，然后读取设备的响应并写入到缓存中
    i2c.write_read(MAGNETOMETER_ADDR, &[MAGNETOMETER_ID_REG], &mut mag).unwrap();//

    rprintln!("The accelerometer chip's id is: {:#b}", acc[0]);
    rprintln!("The magnetometer  chip's id is: {:#b}", mag[0]);

    loop {}
}