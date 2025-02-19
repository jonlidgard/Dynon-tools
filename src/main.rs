#![allow(unused)]
use core::str;
use std::io::{self, Write};
use std::process::exit;
use std::time::Duration;
use std::thread;

use clap::{Arg, Command};

use dynon_data_defs::{DynonSerializable, DynonSerializedData};
use serialport::{DataBits, SerialPort, StopBits};

mod dynon_data_defs;

const EFIS_DATA_OPTION: &str = "efis";
const EMS_DATA_OPTION: &str = "ems";

const ABOUT: &str = "

Write dummy efis or ems data frames to a serial port.

- Defauls to 115200 baud & 1Hz transmission rate which
  is the Dynon EFIS D100 & EMS D120 max rate.

- THE TIME VALUE NEEDS CHANGING TO A PURE INCREMENT NOT THE
  FUNCTION CURRENTLY USED.
";


fn main() {
    let matches = Command::new("Serialport Example - Heartbeat")
        .about(ABOUT)
        .disable_version_flag(true)
        .arg(
            Arg::new("port")
                .help("The device path to a serial port")
                .required(true),
        )
        .arg(
            Arg::new("type")
                .value_parser([EFIS_DATA_OPTION, EMS_DATA_OPTION])
                .default_value("ems")
                .help("The type of data to send - ems or efis")
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

    let port_name = matches.get_one::<String>("port").unwrap();
    let data_type: &String = matches.get_one::<String>("type").expect("default");
    let baud_rate = matches.get_one::<u32>("baud").unwrap(); //.parse::<u32>().unwrap();
    let stop_bits = StopBits::One;
    let data_bits = DataBits::Eight;
    let rate = matches.get_one::<u32>("rate").unwrap(); //.parse::<u32>().unwrap();
    
    //println!("Available ports: {:#?}", serialport::available_ports());
    let builder = serialport::new(  port_name, *baud_rate)
        .stop_bits(stop_bits)
        .data_bits(data_bits);
    //println!("{:?}", &builder);

    let mut port = builder.open().unwrap_or_else(|e| {
        eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
        ::std::process::exit(1);
    });

    println!("THE TIME VALUE NEEDS CHANGING TO A PURE INCREMENT NOT THE
  FUNCTION CURRENTLY USED.\n \
        Sending Dynon D1X0 {} data to {} at {} baud at {}Hz",
        &data_type.to_uppercase(), &port_name, &baud_rate, &rate
    );

    /*
        Idea is to generate some serial data, output at a user-defined rate in order to test out the datalogger.
        Time will increment by 1 each transmission to allow detection of missing frames.
        rpm & oil pressure will vary sinusoidally over 100 frames
        everything else can remain static for now.    
     */

    let mut frame_time = 1;
    //let mut ems_frame: dynon_data_defs::D1x0EMSData;

    match data_type.as_str() {
        EFIS_DATA_OPTION => send_efis(port, rate),
        EMS_DATA_OPTION => send_ems(port, rate),
        _ => { eprintln!("Invalid type specified: use either efis or ems"); ::std::process::exit(1);}
    }
        
}

fn valid_baud(val: &str) -> std::result::Result<(), String> {
    val.parse::<u32>()
        .map(|_| ())
        .map_err(|_| format!("Invalid baud rate '{}' specified", val))
}

fn send_ems(mut port: Box<dyn SerialPort>, rate: &u32) {
    let mut frame_time = 1;
    let r = *rate;
//    thread::spawn( move || {
        
    loop {
        let ems_frame = dynon_data_defs::D1x0EMSData::new(frame_time, Some(100));
        frame_time +=1;
        let dynon_serialized_data = &ems_frame.serialize();
        match dynon_serialized_data {
            DynonSerializedData::D1x0Ems(bytes) => {
                match port.write_all(&bytes[0..]) {
                    Ok(_) => (), 
                   // {
                        //print!("{}", byte_string);
                   //     std::io::stdout().flush().unwrap();
                  //  }
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => panic!("Error while eriting data to the port: {}", e), // {eprintln!("{:?}", e); ::std::process::exit(1);},
                }
                if r == 0 {
                    return;
                }
            },
            _ => (),
        }

        std::thread::sleep(Duration::from_millis((1000.0 / (r as f32)) as u64));
    }
  //      });

}

fn send_efis(mut port: Box<dyn SerialPort>, rate: &u32) {
    let mut frame_time = 1;
    let r = *rate;
 //   thread::spawn( move || {
    loop {
        let efis_frame = dynon_data_defs::D1x0EFISData::new(frame_time, Some(100));
        frame_time +=1;
        let dynon_serialized_data = &efis_frame.serialize();
        match dynon_serialized_data {
            DynonSerializedData::D1x0Efis(bytes) => {
                match port.write_all(&bytes[0..]) {
                    Ok(_) => (), 
                   // {
                        //print!("{}", byte_string);
                   //     std::io::stdout().flush().unwrap();
                  //  }
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => panic!("Error while eriting data to the port: {}", e), // {eprintln!("{:?}", e); ::std::process::exit(1);},
                }
/*                
                match port.write( &bytes[0..]) {
                    Ok(_) => {
                        //print!("{}", byte_string);
                        std::io::stdout().flush().unwrap();
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => {eprintln!("{:?}", e); ::std::process::exit(1);},
                }
*/
                if r == 0 {
                    return;
                }
            },
            _ => (),
        }
        std::thread::sleep(Duration::from_millis((1000.0 / (r as f32)) as u64));
    }
//     });
}
  
    
