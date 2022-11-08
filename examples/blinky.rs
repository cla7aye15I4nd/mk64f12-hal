#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use mk64f12_hal as hal;

#[entry]
fn main() -> ! {
    let dp = hal::Peripherals::take().unwrap();

    // Initialize Pins
    dp.SIM.scgc5.modify(|_, w| w.porta().set_bit());
    dp.SIM.scgc5.modify(|_, w| w.portb().set_bit());
    
    dp.GPIOB.pcor.write(|w| w.ptco22().set_bit());
    dp.GPIOB.pddr.modify(|_, w| w.pdd22().set_bit());

    dp.PORTA.pcr2.modify(|_, w| w.mux()._111());

    dp.PORTA.pcr2.modify(|_, w| w
        .ps().clear_bit()
        .pe().clear_bit()
        .dse().clear_bit()
        .isf().clear_bit());

    dp.PORTA.pcr22.modify(|_, w| w.mux()._001());
    
    // Initialize Clock
    dp.SIM.clkdiv1.write(|w| unsafe { w.bits(0x1240000) });

    dp.OSC.cr.modify(|_, w| w
        .sc2p().clear_bit()
        .sc4p().clear_bit()
        .sc8p().clear_bit()
        .sc16p().clear_bit());
    
    dp.MCG.c2.modify(|_, w| w
        .erefs().clear_bit()
        .hgo().clear_bit()
        .range()._10());

    dp.OSC.cr.modify(|_, w| w
        .erclken().set_bit()
        .erefsten().clear_bit());

    loop {
        // dp.GPIOB.ptor.write(|w| w.ptto22().set_bit());
    }
}
