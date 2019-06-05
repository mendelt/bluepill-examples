#![no_main]
#![no_std]

use cortex_m_rt::{entry};
use cortex_m_semihosting::hprintln;
use hal::prelude::*;
use hal::stm32;
use hal::timer::Timer;
use nb::block;

#[allow(unused_imports)]
use panic_semihosting;

#[entry]
fn main() -> ! {
    let peripherals = cortex_m::Peripherals::take().unwrap();
    let device = stm32::Peripherals::take().unwrap();

    let mut flash = device.FLASH.constrain();
    let mut rcc = device.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioc = device.GPIOC.split(&mut rcc.apb2);
    let mut blink = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    let mut timer = Timer::syst(peripherals.SYST, 1.hz(), clocks);

    // Wait for the timer to trigger an update and change the state of the LED
    loop {
        block!(timer.wait()).unwrap();
        blink.set_high();
        block!(timer.wait()).unwrap();
        blink.set_low();
    }
}
