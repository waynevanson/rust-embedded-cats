#![no_std]
#![no_main]

mod servo;

use arduino_hal::{delay_ms, entry, pins, Peripherals};
use embedded_time::duration::{Hours, Minutes};
use panic_halt as _;
use servo::ServoTimer1Pwm;

const OPEN: Hours = Hours(8);
const CLOSE: Hours = Hours(14);
const DELAY_MINUTES: u8 = 8;

// Adapted from this example
// https://github.com/Rahix/avr-hal/blob/3c02df9df80e7585765644a87076680a2d99b29a/examples/arduino-uno/src/bin/uno-manual-servo.rs#L22-L47
//
// This has been adapted to the correct pin of the Adafruit Trinket Pro,
// which is easy enough to do because it uses the same microcontroller chip
// as the Arduino Uno: avr-atmega328p.
#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let pins = pins!(peripherals);

    // IMPORTANT - sets a DDR register.

    let servo = ServoTimer1Pwm::new(pins.d9.into_output(), peripherals.TC1);

    // duration, keep track of how far into the day we are,
    // starting at midnight.
    let mut duration: Minutes = Minutes(0);

    loop {
        // open cat door if it should be open, otherwise close.
        // These values specific to SG90 servo, experiment to find your ones.
        if duration >= OPEN && duration <= CLOSE {
            // move to highest position
            servo.set_duty(650);
        } else {
            // move to lowest position
            servo.set_duty(65);
        }

        servo.enable();

        // Give the servo time to adjust
        delay_ms(1000);

        servo.disable();

        // Increment duration by the delay
        duration.0 += DELAY_MINUTES as u32;
        delay_minutes(DELAY_MINUTES as usize);
    }
}

fn delay_minutes(minutes: usize) {
    for _ in 0..minutes {
        delay_ms(60_000)
    }
}
