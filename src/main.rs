use std::io::{Read, Write};
use std::process::exit;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use log::{debug, error, info, LevelFilter};
use rppal::gpio::Gpio;

const GPIO_M0: u8 = 22;
const GPIO_M1: u8 = 27;
/// In Order:
/// 0xC2 Command
/// 0x00 Begin Index
/// 0x09 Length in byte
/// 0xFF ADDH
/// 0xFF ADDL
/// 0x00 NetID
/// 0x62 (0110 0010) (011 + 00 + 010) Baud rate, Parity bit, Wireless Air Speed
/// 0x00 (0000 0000) (00 + 0 + 000 (Reserved) + 00) Setting of dividing packet, Enable ambient noise, Reserved, Transmit Power
/// 0x17 (23 Decimal) Channel control (410 + value) (410 + 23 => 433 Mhz)
/// 0x03 (0000 0011) (0 + 0 + 0 + 0 + 0 + 011) Enable RSSI, Transmit mode, Relay function, Enable LBT, WOR Mode, WOR Period
/// 0x00 High bytes of Key
/// 0x00 Low bytes of key
const CONFIG_REGISTER: [u8; 12] = [0xC2, 0x00, 0x09, 0xFF, 0xFF, 0x00, 0x62, 0x00, 0x17, 0x03, 0x00, 0x00];

fn main() {
    simple_logger::SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .env()
        .init()
        .unwrap();
    let push_url = std::env::var("PUSH_URL").expect("Expected PUSH_URL to be set");

    info!("Getting pins");
    let gpio = Gpio::new().expect("Error getting GPIO");
    let mut m0_pin = gpio.get(GPIO_M0).expect("Error getting Pin").into_output();
    let mut m1_pin = gpio.get(GPIO_M1).expect("Error getting Pin").into_output();
    m0_pin.set_low();
    m1_pin.set_high();
    debug!("Set pins");
    sleep(Duration::from_secs(1));

    info!("Getting serial port");
    let mut serialport = serialport::new("/dev/ttyS0", 9600)
        .timeout(Duration::from_millis(100))
        .open()
        .expect("Failed to open serialport");
    serialport.write_all(&CONFIG_REGISTER).expect("Couldn't configure LoRa Hat");
    let mut serial_buf: [u8; 12] = [0; 12];
    serialport.read_exact(&mut serial_buf).expect("Couldn't read response");
    if serial_buf.first().unwrap_or(&0) != &0xC1 {
        error!("Did not successfully set registers: {:#04x?}", serial_buf);
        exit(-1);
    }
    m1_pin.set_low();
    sleep(Duration::from_secs(1));

    debug!("Spawning thread");
    thread::spawn(move || {
        loop {
            debug!("Doing request");
            match reqwest::blocking::get(&push_url) {
                Ok(res) => {
                    if res.status() != 200 {
                        error!("Got unexpected status {} from request", res.status())
                    }
                }
                Err(err) => error!("Could not make request: {}", err)
            };
            sleep(Duration::from_secs(30));
        }
    });
}
