#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, prelude::*, Delay, Leds};
use f3::led::Direction::*;

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, Leds) = aux5::init();

    let half_period = 500_u16;
    let directions = [South as usize, North as usize, East as usize, West as usize];

    loop {
        directions
            .iter()
            .for_each(|&d| {
                leds[d].on();
                delay.delay_ms(half_period);
                leds[d].off();
            });
    }
}
