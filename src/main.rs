/*
    Idea is to generate some serial data, output at a user-defined rate in order to test out the datalogger.
    Time will increment by 1 each transmission to allow detection of missing frames.
    rpm & oil pressure will vary sinusoidally over 100 frames
    everything else can remain static for now.
 */


#![allow(unused)]
use core::str;
use std::io::{self, Write};
use std::process::exit;
use std::time::Duration;
use std::thread;
use std::vec;
use clap::{Arg, Command};
use serialport::{DataBits, SerialPort, StopBits};
use crate::dynon_device::*;
use crate::dynon_device::d1x0efis::D1x0EFISDevice;
use crate::dynon_device::d1x0ems::D1x0EMSDevice;

pub mod dynon_device;


const EFIS_DATA_OPTION: &str = "efis";
const EMS_DATA_OPTION: &str = "ems";
const TEST_DATA_OPTION: &str = "test";

const ABOUT: &str = "

Write dummy efis or ems data frames to a serial port.

- Defauls to 115200 baud & 1Hz transmission rate which
  is the Dynon EFIS D100 & EMS D120 max rate.

- The time value updates incrementally & does not relate
  to a system time.
";

fn valid_baud(val: &str) -> std::result::Result<(), String> {
    val.parse::<u32>()
        .map(|_| ())
        .map_err(|_| format!("Invalid baud rate '{}' specified", val))
}


fn send_packets(dynon_device: &mut impl DynonDevice, mut port: Option<Box<dyn SerialPort>>, rate: &u32) {
    let r = *rate;
    loop {
        let (eol, bytes) = dynon_device.as_bytes();
        match str::from_utf8(&bytes) {
            Ok(s) => {
                print!("{}", s);
                io::stdout().flush();
            }
            Err(e) => {
                eprintln!("Invalid UTF-8 sequence: {}", e);
            }
        }
        if let Some(ref mut port) = port {
            match port.write_all(bytes) {
                Ok(_) => (),
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => panic!("Error while writing data to the port: {}", e),
            }
        }
        if (!eol) {
            std::thread::sleep(Duration::from_millis(220));
        }
        else {
            if r == 0 {
                break;
            }
            else {
                std::thread::sleep(Duration::from_millis((1000.0 / (r as f32)) as u64));
               // println!("");
            }
        }
    }
}


fn main() {
    let matches = Command::new("Serialport Example - Heartbeat")
        .about(ABOUT)
        .disable_version_flag(true)
        .arg(
            Arg::new("type")
                .value_parser([EFIS_DATA_OPTION, EMS_DATA_OPTION, TEST_DATA_OPTION])
                .default_value("ems")
                .help("The type of data to send - ems or efis")
        ).arg(
            Arg::new("port")
                .long("port")
                .help("The device path to a serial port")
                .required(false),
        ).arg(
            Arg::new("baud")
                .long("baud")
                .help("The baud rate to connect at")
                .use_value_delimiter(false)
                .default_value("115200")
                .value_parser(clap::value_parser!(u32).range(9600..)),
        )
        .arg(
            Arg::new("rate")
                .long("rate")
                .help("Frequency (Hz) to repeat transmission of the pattern (0 indicates sending only once")
                .num_args(1)
                .default_value("1")
                .value_parser(clap::value_parser!(u32)),
        )
        .get_matches();

    let data_type: &String = matches.get_one::<String>("type").expect("default");
    let baud_rate = matches.get_one::<u32>("baud").unwrap(); //.parse::<u32>().unwrap();
    let rate = matches.get_one::<u32>("rate").unwrap(); //.parse::<u32>().unwrap();
    let mut port: Option<Box<dyn SerialPort>> = Option::None;
    let port_name = matches.get_one::<String>("port");
    if let Some(port_name) = port_name {

        let builder = serialport::new(  port_name, *baud_rate)
            .stop_bits(StopBits::One)
            .data_bits(DataBits::Eight);

        port = Some(builder.open().unwrap_or_else(|e| {
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
            ::std::process::exit(1);
        }));
    }
    // println!("Available ports: {:#?}", serialport::available_ports());
    //println!("{:?}", &builder);

    println!(
        "Sending Dynon D1X0 {} data at {} baud at {}Hz",
        &data_type.to_uppercase(), &baud_rate, &rate
    );

    match data_type.as_str() {
        EFIS_DATA_OPTION => send_packets(&mut D1x0EFISDevice::new(Some(100)), port, rate),
        EMS_DATA_OPTION => send_packets(&mut D1x0EMSDevice::new(Some(100)), port, rate),
        TEST_DATA_OPTION => send_packets(&mut TestDevice::new(Some(100)), port, rate),
        _ => { eprintln!("Invalid type specified: use either efis or ems"); ::std::process::exit(1);}
    }

}
