#![no_main]
#![no_std]
use core::fmt::Write;
use heapless::Vec;

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use microbit::{
    hal::prelude::*,
    hal::uarte::{Baudrate, Parity, Pins, Uarte, UarteTx},
    hal::{twim::Twim, uarte::UarteRx},
    pac::twim0::frequency::FREQUENCY_A,
    pac::{TWIM0, UARTE0},
};

use lsm303agr::{
    interface::I2cInterface, mode::MagOneShot, AccelOutputDataRate, Lsm303agr, MagOutputDataRate,
};

static mut TX_BUF: [u8; 1] = [0; 1];
static mut RX_BUF: [u8; 1] = [0; 1];

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();
    //初始化uart设备
    let (mut tx, mut rx) = get_tx_and_rx(
        board.UARTE0,
        board.uart.into(),
        Parity::EXCLUDED,
        Baudrate::BAUD115200,
    );
    //初始化i2c设备
    let i2c = Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100);
    let sensor = init_sensor(i2c);
    let mut sensor = sensor.into_mag_continuous().ok().unwrap();
    let mut buffer: Vec<u8, 32> = Vec::new();
    loop {
        //阻塞读取uart设备
        let byte = nb::block!(rx.read()).unwrap();
        if byte == 0x0D {
            //判断读取到的内容
            let input = core::str::from_utf8(&buffer).unwrap();
            rprintln!("输入内容:{}", input);
            match input {//跟i2c通讯
                "a" => {
                    while !sensor.accel_status().unwrap().xyz_new_data {} // 等待直到数据就绪
                    let data = sensor.accel_data().unwrap();
                    writeln!(//回显读取到的内容
                        &mut tx,
                        "Acceleration: x {} y {} z {}\r",
                        data.x, data.y, data.z
                    ).unwrap();
                }
                "m" => {
                    while !sensor.mag_status().unwrap().x_new_data {} // 等待直到数据就绪
                    let data = sensor.mag_data().unwrap();
                    writeln!(
                        &mut tx,
                        "Magnetometer: x {} y {} z {}\r",
                        data.x, data.y, data.z
                    ).unwrap();
                }
                _ => writeln!(&mut tx, "命令无法识别:{}\r", input).unwrap(),
            };

            buffer.clear();
            continue;
        }
        if buffer.push(byte).is_err() {
            writeln!(&mut tx, "输入过长\r").unwrap();
            buffer.clear();
        }
    }
}

fn get_tx_and_rx(
    uarte: UARTE0,
    pins: Pins,
    parity: Parity,
    baudrate: Baudrate,
) -> (UarteTx<UARTE0>, UarteRx<UARTE0>) {
    let uarte_instance = Uarte::new(uarte, pins, parity, baudrate);
    uarte_instance
        .split(unsafe { &mut TX_BUF }, unsafe { &mut RX_BUF })
        .unwrap()
}

fn init_sensor(i2c: Twim<TWIM0>) -> Lsm303agr<I2cInterface<Twim<TWIM0>>, MagOneShot> {
    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz50).unwrap();
    sensor.set_mag_odr(MagOutputDataRate::Hz50).unwrap();
    sensor
}
