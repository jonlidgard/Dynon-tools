#![allow(unused)]
#[cfg(test)]

extern crate rand;

use crate::dynon_device::*;

const D1X0_EMS_FRAME_LENGTH: usize = 121;
type D1x0EmsFrame = [u8; D1X0_EMS_FRAME_LENGTH];

#[derive(Debug)]
pub struct D1x0EMSDevice {
    bytes: D1x0EmsFrame,
    frame_start: usize,
    freq: u32,
    system_time: u32,  // HHMMSSFF
    manifold_pressure: f32,  // unit: inHg * 100 (4)
    oil_temp: i16,  // unit : deg celcius / XXX (3)
    oil_pressure: u16,  // unit : psi / (3)
    fuel_pressure: f32,  // unit: PSI / n * 10 / XXX (3)
    volts: f32,  // n * 10 / (3)
    amps: f32,  // n * 10 / (3)
    rpm: u16,  // n / 10 (3)
    fuel_flow: f32,  // unit: gallons per hour / n * 10 / (3)
    fuel_remaining: f32,  // unit: gallons / calculated by fuel computer / n * 10 / XXX (4)
    fuel_level_1: f32,  // unit: gallons / if MAIN tank exists, it will be MAIN / n * 10 / XXX (3)
    fuel_level_2: f32,  // unit: gallons / n * 10 / XXX (3)
    gp_1: [char; 8],  // unit: variable / n * 10 / +- value unit / +1234C (8)
    gp_2: [char; 8],  // unit: variable / n * 10 / +- value unit / +1234C (8)
    gp_3: [char; 8],  // unit: variable / n * 10 / +- value unit / +1234C (8)
    gp_thermocouple: i16, // (4)
    egt_1: i16, //(4)
    egt_2: i16, //(4)
    egt_3: i16, //(4)
    egt_4: i16, //(4)
    egt_5: i16, //(4)
    egt_6: i16, //(4)
    cht_1: i16, //(3)
    cht_2: i16, //(3)
    cht_3: i16, //(3)
    cht_4: i16, //(3)
    cht_5: i16, //(3)
    cht_6: i16, //(3)
    contact_1: u16, //(1)
    contact_2: u16, //(2)
}


impl D1x0EMSDevice {
    pub fn new(freq: Option<u32>) -> Self {
        Self {
            bytes: [b'0'; D1X0_EMS_FRAME_LENGTH],
            frame_start: rng().random_range(0..D1X0_EMS_FRAME_LENGTH),
            freq: freq.unwrap_or(100),
            system_time: 1,
            manifold_pressure: 0.0,
            oil_temp: 0,
            oil_pressure: 0 as u16,
            fuel_pressure: 0.0,
            volts: 12.0,
            amps: 5.0,
            rpm: 0 as u16,
            fuel_flow: 0.0,
            fuel_remaining: 0.0,
            fuel_level_1: 0.0,
            fuel_level_2: 0.0,
            gp_1: ['X'; 8],
            gp_2: ['X'; 8],
            gp_3: ['X'; 8],
            gp_thermocouple: 0,
            egt_1: 0,
            egt_2: 0,
            egt_3: 0,
            egt_4: 0,
            egt_5: 0,
            egt_6: 0,
            cht_1: 0,
            cht_2: 0,
            cht_3: 0,
            cht_4: 0,
            cht_5: 0,
            cht_6: 0,
            contact_1: 0,
            contact_2: 0
        }
    }

    fn calc_crc(data: &[u8]) -> u8 {
        let sum: u64 = data[..data.len()-4].iter().map(|x| *x as u64).sum();
        (0x100 - (sum & 0xff)) as u8
    }

    fn update(&mut self) {
        let f = self.freq;
        let phase: f32 = (( self.system_time as f32 % f as f32 ) * 180.0 / f as f32).to_radians();
        //println!("Phase: {}", phase);
        // system_time
        let time_string = format!("{:0>8}", self.system_time.to_string());
        self.bytes[..SYSTEM_TIME_LENGTH].copy_from_slice(time_string.as_bytes());

        // oil pressure
        self.oil_pressure = (50.0 * f32::sin(phase)) as u16;
        let oil_pressure = format!("{:0>3}", self.oil_pressure.to_string());
        self.bytes[15 .. 18].copy_from_slice(oil_pressure.as_bytes());

        // rpm
        self.rpm = (3000.0 * f32::sin(phase)) as u16;
        let rpm = format!("{:0>3}", (self.rpm / 10).to_string());
        self.bytes[27 .. 30].copy_from_slice(rpm.as_bytes());

        // GP1-3 as unused 'XXXXXXXX'
        self.bytes[43 .. 67].copy_from_slice("XXXXXXXXXXXXXXXXXXXXXXXX".as_bytes());

        // crc
        let crc = Self::calc_crc(&self.bytes);
        Self::add_crc_crlf(crc, &mut self.bytes);
        self.system_time += 1;
    }
}

impl DynonDevice for D1x0EMSDevice {

    fn as_bytes(&mut self) -> (bool, &[u8]) {
        let mut rng = rng();
        let slice_start = self.frame_start;
        let slice_end = rng.random_range((slice_start+1)..D1X0_EMS_FRAME_LENGTH+1);
        self.frame_start = slice_end;
        if self.frame_start >= D1X0_EMS_FRAME_LENGTH {
            self.frame_start = 0;
            self.update();
        }
        (self.frame_start == 0, &self.bytes[slice_start..slice_end])
    }
}

//---------------------------

#[test]
fn test_d1x0_ems_calc_crc() {
    let mut x: D1x0EmsFrame = [0; D1X0_EMS_FRAME_LENGTH];
    x.copy_from_slice("0012224826351340262441240122631320562191191OAT00090TRE-0061FLP0001020481378139214061421143514503583533633743843951103D200".as_bytes());

    let y = D1x0EMSData::calc_crc(&x);
    //println!("Y: {:?}",y);
    assert_eq!(y,0xD2);
}

#[test]
fn test_d1x0_ems_serialize() {
    let x = D1x0EMSData::new(12345678, None);
    println!("X: {:?}", x);
    let d = x.serialize();
    match d {
        DynonSerializedData::D1x0Ems(y) => {
            println!("Y: {:?}",str::from_utf8(&y));
            assert_eq!(&y[..8], "12345678".as_bytes());
            let l = y.len();
            assert_eq!(&y[l-4 .. l-2], "1D".as_bytes());
        }
        _ => panic!("Returned wrong type."),
    }
}
