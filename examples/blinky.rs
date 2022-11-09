#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::entry;
use mk64f12_hal as hal;


fn boot_init_pins(dp : &mut hal::Peripherals) {
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
}

fn boot_init_clock(dp : &mut hal::Peripherals) {
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
    
    let mcg_out_clk_state = dp.MCG.s.read().clkst().bits();
    let cur_ircs = dp.MCG.s.read().ircst().bit();
    let cur_fcrdiv = dp.MCG.sc.read().fcrdiv().bits();

    if cur_fcrdiv != 0 {
        if (false != dp.MCG.c1.read().irclken().bit() || mcg_out_clk_state == 1) && cur_ircs == true {
            dp.MCG.c2.modify(|_, w| w
                .ircs().clear_bit());

            while dp.MCG.s.read().ircst().bit() != false {}
        }

        dp.MCG.sc.modify(|_, w| w
            .fcrdiv()._000()
            .atmf().clear_bit()
            .locs0().clear_bit());
    }

    dp.MCG.c2.modify(|_, w| w.ircs().clear_bit());
    dp.MCG.c1.modify(|_, w| w
        .irclken().set_bit()
        .irefsten().clear_bit());
    
    while false != dp.MCG.s.read().ircst().bit() {}

    dp.MCG.c1.modify(|_, w| w.frdiv()._000());

    let need_delay = dp.MCG.c7.read().oscsel().bits() != 0;

    dp.MCG.c7.modify(|_, w| w.oscsel()._00());
    if need_delay {
        for _ in 0..1500 {}
    }

    dp.MCG.c2.modify(|_, w| w.lp().clear_bit());
    dp.MCG.c1.modify(|_, w| w
        .clks()._10()
        .irefs().clear_bit());
    
    loop {
        let val = dp.MCG.s.read();
        if val.irefst().bit() == false && val.clkst().bits() == 2 { break;}
    }

    dp.MCG.c6.modify(|_, w| w.plls().clear_bit());
    while dp.MCG.s.read().pllst().bit() != false {}

    dp.MCG.c5.modify(|_, w| w.prdiv0()._19());
    dp.MCG.c6.modify(|_, w| w.vdiv0()._24());
    dp.MCG.c5.modify(|_, w| w.pllclken0().set_bit());

    while false == dp.MCG.s.read().lock0().bit() {}

    dp.MCG.c6.modify(|_, w| w.plls().set_bit());
    while dp.MCG.s.read().pllst().bit_is_clear() {}

    dp.MCG.c1.modify(|_, w| w.clks()._00());
    while !dp.MCG.s.read().clkst().is_11() {}

    dp.SIM.clkdiv1.write(|w| unsafe { w.bits(0x1240000) });
    dp.SIM.sopt2.modify(|_, w| w.pllfllsel()._01());
    dp.SIM.sopt1.modify(|_, w| w.osc32ksel()._10());

}

#[entry]
fn main() -> ! {
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let mut dp = hal::Peripherals::take().unwrap();
    
    dp.WDOG.unlock.write(|w| unsafe { w.wdogunlock().bits(0xc520) } );
    dp.WDOG.unlock.write(|w| unsafe { w.wdogunlock().bits(0xd928) } );
    dp.WDOG.stctrlh.write(|w| unsafe { w.bits(0x1d2) });

    boot_init_pins(&mut dp);
    boot_init_clock(&mut dp);

    cp.SYST.set_reload(120000 - 1);
    cp.SYST.clear_current();
    cp.SYST.set_clock_source(SystClkSource::Core);
    cp.SYST.enable_counter();

    loop {
        if cp.SYST.has_wrapped() {
            dp.GPIOB.ptor.write(|w| w.ptto22().set_bit());
        }
    }
}
