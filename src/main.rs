use embedded_hal::delay::DelayNs;
use esp_idf_hal::delay::Ets;
use esp_idf_hal::gpio::AnyIOPin;
use esp_idf_hal::i2c::{I2cConfig, I2cDriver};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;

fn main() -> ! {
    println!("start!");
    let peripherals = Peripherals::take().unwrap();
    let gpios = peripherals.pins;
    let mut delay = Ets;

    let sda: AnyIOPin = gpios.gpio2.into();
    let scl: AnyIOPin = gpios.gpio1.into();

    let i2c_config = I2cConfig::new().baudrate(100.kHz().into());
    let mut i2c_driver = I2cDriver::new(peripherals.i2c0, sda, scl, &i2c_config).unwrap();

    loop {
        delay.delay_ms(2000);

        println!("\nI2C Bus Scan");
        println!("     0   1   2   3   4   5   6   7   8   9   A   B   C   D   E   F");
        let mut count = 0;

        for addr in 0..127u8 {
            let mut data = [0u8, 1];
            let result = i2c_driver.read(addr, &mut data, 1);

            if addr % 16 == 0 {
                print!("{}0", count);
                count += 1;
            }

            match result {
                Ok(_) => {
                    print!(" {:02X} ", addr);
                }
                Err(_) => {
                    print!("  . ");
                }
            }

            if addr % 16 == 15 {
                println!();
            }
        }
        println!();
        println!("finished.");
    }
}
