#![no_std]
#![no_main]

mod tsl2561_lux;

use arduino_hal::{simple_pwm::*, *};
use panic_halt as _;
use tsl2561_lux::{into_lux, PackageCoefficient};
use tsl256x::{Gain, IntegrationTime, SlaveAddr, Tsl2561};

fn delay_minutes(minutes: usize) {
    let minute_ms = 1000 * 60;
    for _ in 0..minutes {
        delay_ms(minute_ms)
    }
}

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let pins = pins!(peripherals);

    // I2C
    let sda = pins.a4.into_pull_up_input();
    let sdc = pins.a5.into_pull_up_input();
    let speed = 100_000;
    let mut i2c = I2c::new(peripherals.TWI, sda, sdc, speed);

    // Light sensor
    let light_sensor = Tsl2561::new(&i2c, SlaveAddr::default().addr()).unwrap();

    // Servo motor
    let timer = Timer0Pwm::new(peripherals.TC0, Prescaler::Prescale64);
    let mut servo_pin = pins.d5.into_output().into_pwm(&timer);

    let lux_to_open = 400;
    let lux_to_close = 500;

    let servo_open = 255;
    let servo_close = 0;

    let integration_time = IntegrationTime::ms_402;
    let gain = Gain::Low;

    loop {
        // enabling/disabling in loop to save power.
        servo_pin.enable();
        light_sensor.power_on(&mut i2c).unwrap();

        // set sane defaults
        light_sensor
            .config_time_gain(&mut i2c, integration_time, gain)
            .unwrap();

        // time for light sensor integration cycle (400ms)
        // so above settings can be applied.
        delay_ms(400);

        let broadband_level_0 = light_sensor.visible_and_ir_raw(&mut i2c).unwrap();
        let infrared_level_1 = light_sensor.ir_raw(&mut i2c).unwrap();
        let lux_level = into_lux(
            broadband_level_0,
            infrared_level_1,
            integration_time,
            gain,
            PackageCoefficient::T,
        );

        // good morning angels, lets go outside (in the sunshine).
        if lux_level >= lux_to_open {
            servo_pin.set_duty(servo_open)
        }

        // come inside, angels. It's getting dark..
        if lux_level <= lux_to_close {
            servo_pin.set_duty(servo_close);
        }

        // enabling/disabling in loop to save power.
        light_sensor.power_off(&mut i2c).unwrap();
        servo_pin.disable();

        delay_minutes(5);
    }
}
