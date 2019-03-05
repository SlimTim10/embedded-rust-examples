#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux8::{entry, rcc, gpioc};

fn power_on_gpioe(rcc: &rcc::RegisterBlock) -> () {
    rcc.ahbenr.modify(|_, w| w.iopeen().set_bit());
}

fn set_leds_output(gpioe: &gpioc::RegisterBlock) -> () {
    gpioe.moder.modify(|_, w| {
        w.moder8().output();
        w.moder9().output();
        w.moder10().output();
        w.moder11().output();
        w.moder12().output();
        w.moder13().output();
        w.moder14().output();
        w.moder15().output()
    });
}

fn turn_on_leds(gpioe: &gpioc::RegisterBlock) -> () {
    gpioe.odr.write(|w| {
        w.odr8().set_bit();
        w.odr9().set_bit();
        w.odr10().set_bit();
        w.odr11().set_bit();
        w.odr12().set_bit();
        w.odr13().set_bit();
        w.odr14().set_bit();
        w.odr15().set_bit()
    });
}

#[entry]
fn main() -> ! {
    let (gpioe, rcc) = aux8::init();

    power_on_gpioe(rcc);
    set_leds_output(gpioe);

    turn_on_leds(gpioe);

    aux8::bkpt();

    loop {}
}
