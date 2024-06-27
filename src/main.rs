use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::hal::prelude::Peripherals;

fn main() {
    esp_idf_svc::sys::link_patches();

    let peripherals = Peripherals::take();
    let mut led = PinDriver::output(peripherals.unwrap().pins.gpio2).unwrap(); //::Output(peripherals.unwrap().pins.gpio48).unwrap();

    loop {
        led.set_high().expect("something wrong has happened");
        FreeRtos::delay_ms(2000);
        led.set_low().expect("something wrong happened");
        FreeRtos::delay_ms(500);
    }
}
