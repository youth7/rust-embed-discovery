#![no_main]
#![no_std]

use core::fmt::Write;
use core::str;
use cortex_m_rt::entry; // cortex-m的运行时
use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};
use heapless::Vec;

use microbit::{
    hal::prelude::*,
    hal::uarte::{Baudrate, Parity, Uarte, UarteTx}, pac::{UARTE0},
};

static mut TX_BUF: [u8; 1] = [0; 1];
static mut RX_BUF: [u8; 1] = [0; 1];

#[entry]
fn main() -> ! {
    rtt_init_print!(); //初始化rtt
    let board = microbit::Board::take().unwrap();
    let uarte_instance = Uarte::new(
        //构造一个uarte实例
        board.UARTE0,
        board.uart.into(),
        Parity::EXCLUDED,
        Baudrate::BAUD115200,
    );
    //将uarte实例上的tx和rx提取出来单独使用
    let (mut tx, mut rx) = uarte_instance
        .split(unsafe { &mut TX_BUF }, unsafe { &mut RX_BUF })
        .unwrap();
    let mut buffer:Vec<u8, 32> = Vec::new();    
    loop {
        //read是非阻塞的，如果没有字符可读的时候会抛出一个异常，
        //为了能够读取到数据需要持续地调用read直到不抛异常为止，而nb::block其实就是做了这个事情
        let byte = nb::block!(rx.read()).unwrap();
        if byte == 0x0D {
            send_str(&mut tx, &mut buffer);
        }else{
            if buffer.push(byte).is_err() {
                send_str(&mut tx, &mut buffer);
                continue;
            }
        }
    }
}

fn send_str(tx: &mut UarteTx<UARTE0>, buffer: &mut Vec<u8, 32>){//发送字符串
    let content = str::from_utf8(&buffer).unwrap();
    rprintln!("{}", content);//在rtt控制台上检查接收的内容
    tx.write_str(content).unwrap();//原封不动回写全部内容
    tx.write_str("\r\n").unwrap();//加上回车换行以便显示
    nb::block!(tx.flush()).unwrap();//刷新缓冲区以免内容残留
    buffer.clear();//清空buffer以接收下一次内容
}