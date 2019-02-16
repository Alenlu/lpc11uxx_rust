#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

extern crate lpc11uxx;

use cortex_m::asm;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {

    let peripherals = lpc11uxx::Peripherals::take().unwrap();
    let gpio = &peripherals.GPIO_PORT;

    // set pin direction to out
    gpio.dir[0].write(|w| w.dirp7().set_bit());

    // turn on led
    gpio.set[0].write(|w| w.setp7().set_bit());

    loop {
        for _x in 0..20_000 {
            asm::nop();
        }

        gpio.not[0].write(|w| w.notp7().set_bit());
    }
}
