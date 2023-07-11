#![no_std]
#![no_main]

use arduino_hal::{delay_ms, entry, pins, Peripherals};
use embedded_time::duration::{Hours, Minutes};
use panic_halt as _;

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
    pins.d9.into_output();

    let tc1 = peripherals.TC1;

    // duration, keep track of how far into the day we are,
    // starting at midnight.
    let mut duration: Minutes = Minutes(0);

    loop {
        // Set up PWM that is compatible with servo motor.
        // This is usually implemented within a struct like `Timer[n]Pwm`,
        // but we'll apply it manually here.
        //
        // servo.enable()

        // open cat door if it should be open, otherwise close.
        // These values specific to SG90 servo, experiment to find your ones.
        if duration >= OPEN && duration <= CLOSE {
            // move to highest position
            tc1.ocr1a.write(|w| w.bits(650));
        } else {
            // move to lowest position
            tc1.ocr1a.write(|w| w.bits(65));
        }

        tc1.icr1.write(|w| w.bits(4999));
        tc1.tccr1a
            .write(|w| w.wgm1().bits(0b10).com1a().match_clear());
        tc1.tccr1b
            .write(|w| w.wgm1().bits(0b11).cs1().prescale_64());
        // Give the servo time to adjust
        delay_ms(1000);

        // servo.disable()
        tc1.tccr1a
            .write(|w| w.wgm1().bits(0b10).com1a().disconnected());

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
