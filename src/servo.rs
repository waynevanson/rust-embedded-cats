use arduino_hal::{
    hal::port::PB1,
    pac::TC1,
    port::{mode::Output, Pin},
};

pub struct ServoTimer1Pwm {
    pin: Pin<Output, PB1>,
    timer: TC1,
}

// This impl hasn't been tested, even if it was copy pasted from a working impl.
impl ServoTimer1Pwm {
    /// Pin<Output, ..> ensures the pin sets the DDR register via `pin.into_output()`
    pub fn new(pin: Pin<Output, PB1>, timer: TC1) -> Self {
        Self { pin, timer }
    }

    /// Turns on the servo motor
    pub fn enable(&self) {
        self.timer.icr1.write(|w| w.bits(4999));

        self.timer
            .tccr1a
            .write(|writer| writer.wgm1().bits(0b10).com1a().match_clear());

        // TODO - Prescaler as parameter
        self.timer
            .tccr1b
            .write(|writer| writer.wgm1().bits(0b11).cs1().prescale_64());
    }

    pub fn disable(&self) {
        self.timer
            .tccr1a
            .write(|writer| writer.wgm1().bits(0b10).com1a().disconnected());
    }

    pub fn set_duty(&self, duty: u16) {
        self.timer.ocr1a.write(|writer| writer.bits(duty))
    }
}
