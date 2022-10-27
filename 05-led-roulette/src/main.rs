#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;//这个和《The embedonomicon》中的entry宏原理是一样的，原理是库函数才是真正的入口，然后库函数再来加载用户函数
use panic_halt as _;

#[entry]
fn main() -> ! {//发散函数，《The embedonomicon》也说过
    let _y;
    let x = 42;
    _y = x;
    // infinite loop; just so we don't leave this stack frame
    loop {}
}