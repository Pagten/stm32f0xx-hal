#![no_main]
#![no_std]

use panic_halt as _;

use stm32f0xx_hal as hal;

use crate::hal::{pac, prelude::*, serial::Serial};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    if let Some(p) = pac::Peripherals::take() {
        cortex_m::interrupt::free(move |cs| {
            let mut flash = p.FLASH;
            let mut rcc = p.RCC.configure().sysclk(48.mhz()).freeze(&mut flash);

            let gpioa = p.GPIOA.split(&mut rcc);

            let tx = gpioa.pa9.into_alternate_af1(cs);
            let rx = gpioa.pa10.into_alternate_af1(cs);

            let mut serial = Serial::usart1(p.USART1, (tx, rx), 115_200.bps(), &mut rcc);

            loop {
                // Wait for reception of a single byte
                let received = nb::block!(serial.read()).unwrap();

                // Send back previously received byte and wait for completion
                nb::block!(serial.write(received)).ok();
            }
        });
    }

    loop {
        continue;
    }
}
