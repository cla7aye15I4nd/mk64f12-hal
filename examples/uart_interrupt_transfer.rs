#![allow(clippy::empty_loop)]

#![no_main]
#![no_std]


use panic_halt as _;

use mk64f12_hal as hal;
use cortex_m_rt::entry;


#[entry]
fn main() -> ! {
    let mut peripherals = hal::Peripherals::take().unwrap();

    loop {}
}