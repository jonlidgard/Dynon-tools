#![allow(unused)]
#[cfg(test)]

extern crate rand;

use core::{str, time};
//use std::slice::range;
use rand::Rng;
use rand::rng;

const D1X0_EMS_FRAME_LENGTH: usize = 121;
const D1X0_EFIS_FRAME_LENGTH: usize = 53;
const SYSTEM_TIME_LENGTH: usize = 8;

type D1x0EmsFrame = [u8; D1X0_EMS_FRAME_LENGTH];
type D1x0EfisFrame = [u8; D1X0_EFIS_FRAME_LENGTH];

pub trait DynonSerialize {
    fn calc_crc(data: &[u8]) -> u8 {
        let sum: u64 = data[..data.len()-4].iter().map(|x| *x as u64).sum();
        (sum & 0xff) as u8
    }

    fn add_crc_crlf(crc: u8, data: &mut[u8]) {
        let l = data.len();

        // CRC
        let crc_str = format!("{crc:02X}");
        //println!("CRC: {:?}, {:?}",crc, crc_str);
        data[l-4 .. l-2].copy_from_slice(crc_str.as_bytes());

        // CRLF
        data[l-1] = b'\n';
        data[l-2] = b'\r';
    }

    fn as_bytes(&mut self) -> &[u8];
}


#[derive(Debug)]
pub struct TestData {
    bytes: [u8; 26],
    frame_start: usize,
}

impl TestData {

    pub fn new(time_count: u32, freq: Option<u32>) -> Self {

        const fn create_alphabet_array() -> [u8; 26] {
            let mut arr = [0; 26]; // Initialize with a default value that implements Copy
            let mut i = 0;
            while i < 26 {
                // 'A' as u8 gets the ASCII value, add the index, cast back to char
                arr[i] = (b'A' + i as u8);
                i += 1;
            }
            arr
        }

        const ALPHABET: [u8; 26] = create_alphabet_array();
        Self {
            bytes: ALPHABET,
            frame_start: 0,
        }
    }
}

impl DynonSerialize for TestData {
    fn as_bytes(&mut self) -> &[u8] {

        let mut rng = rng();
        let slice_start = self.frame_start;
        let slice_end = rng.random_range((slice_start+1)..27);
        self.frame_start = slice_end;
        if self.frame_start >= 26 {
            self.frame_start = 0;
        }
        &self.bytes[slice_start..slice_end]
    }
}

/*struct ADAHRSData {
    system_time: u32,  // HHMMSSFF
    pitch: i16,  // unit: deg / n * 10 / positive = pitch up / XXXX is unavailable
    roll: f32,  // unit: deg / n * 10 / positive = right bank / XXXXX
    mag_heading: i16,  // unit: deg / XXX
    ias: f32,  // IAS / unit: knots / n * 10 / XXXX
    pressure_alt: i16,  // unit : feet / at 29.92Hg / can be minus (sea level) / XXXXXX
    turn_rate: i16,  // unit: deg / XXXX
    lateral_accel: f32,  // unit: g / n * 100 / positive = left skid (ball right) / XXX
    vertical_accel: f32,  // unit: g / n * 100 / positive = going up accel / XXX
    aoa: i16,  // AOA / unit: % / XX
    vertical_speed: i16,  // unit: ft/min / positive = climbing / XXX(4)
    oat: i16,  // OAT(Outside Air Temperature) / unit: deg celcius / XXX
    tas: f32,  // TAS / unit: knot / n * 10 / XXXX
    barometer_setting: f32,  // Barometer setting / unit: inHg / n * 100 / offset from 27.50 / XXX
    density_alt: i16,  // unit: feet / positive = above set level / XXXXXX
    wind_dir: i16,  // unit: deg / XXX
    wind_speed: i16  // unit: knots / XX
}


struct SystemData {
    system_time: u32,  // HHMMSSFF
    heading_bug: i16,  // unit: deg / XXX
    altitude_bug: i16,  // unit: feet / n / 10 / XXXXX
    airspeed_bug: f32,  // unit: knots / n * 10 / XXXX
    vertical_speed_bug: i16,  // unit: feet / n * 10 / XXXX
    course: i16,  // unit : deg / (3)
    cdi_src_type: i16,  // CDI Source Type / 0=GPS 1=NAV 2=LOC / (1)
    cdi_src_port: i16,  // 0-5 / GPSX, NAVX, LOCX / (1)
    cdi_scale: i16,  // 00-50 / unit: nm / n * 10 / XX
    cdi_deflection: i16,  // unit: % / positive: deflected right / XXX
    glide_slope: i16,  // unit: % / positive: upward from gs / XXX
    ap_engaged: i16,  // 0-7 / 0=off 1=roll only 2=pitch only 3=roll&pitch 4=yaw 5=roll&yaw 6=pitch&yaw 7=pitch&roll&yaw
    // raw currently not supported / 0 if ap not available / (1)
    ap_roll_mode: i16,  // 0-4 / 0=Heading 1=Track 2=NAV 3=GPS Steering / 0 if ap roll not engaged / (1)
    not_used_1: char,  // X(1)

    // ap_x_force = 0-80 / raw force from servo / max 80 before slip / 0 if ap not engaged / (3)
    // ap_x_pos = 0-9999 / position of servo output / 800 is full rotation output / 0 if ap not engaged / XXXXX
    // ap_x_slip = 0-1 / 0=no slip 1=slip at least 3 seconds / 0 if ap not engaged / (1)
    ap_roll_force: i16,  // 0-80 / positive = right wing downward force / (3)
    ap_roll_pos: i16,  // positive = right wing downward
    ap_roll_slip: i16,
    ap_pitch_force: i16,  // positive = nose up dir
    ap_pitch_pos: i16,  // positive = nose up dir
    ap_pitch_slip: i16,
    ap_yaw_force: i16,  // positive = rightward dir
    ap_yaw_pos: i16,  // positive = rightward dir
    ap_yaw_slip: i16,

    transponder_status: i16,  // 0=SBY 1=GND 2=ON 3=ALT / (1)
    transponder_reply: i16,  // 0=No reply in last second 1=at least 1 reply in last second / (1)
    transponder_code: [char; 4],  // 0000-7777

    not_used_2: [char; 10],  // XXXXXXXXXX(10)
}

struct EMSData {
    system_time: u32,  // HHMMSSFF
    oil_pressure: i16,  // unit : psi / (3)
    oil_temp: i16,  // unit : deg celcius / XXX
    RPM_L: i16,  // (4)
    RPM_R: i16,  // (4)
    manifold_pressure: f32,  // 0-600 / unit: inHg / n * 10 / (3)
    fuel_flow_1: f32,  // unit: gallons per hour / n * 10 / (3)
    fuel_flow_2: f32,  // unit: gallons per hour / n * 10 / (3)
    fuel_pressure: f32,  // unit: PSI / n * 10 / XXX
    fuel_level_L: f32,  // unit: gallons / if MAIN tank exists, it will be MAIN / n * 10 / XXX
    fuel_level_R: f32,  // unit: gallons / n * 10 / XXX
    fuel_remaining: f32,  // unit: gallons / calculated by fuel computer / n * 10 / XXX
    volts_1: f32,  // 000-360 / n * 10 / (3)
    volts_2: f32,  // 000-360 / n * 10 / (3)
    amps: f32,  // n * 10 / (3)
    hobbs_time: f32,  // unit: hours / n * 10 / (5)
    tach_time: f32,  // unit: hours / n * 10 / (5)
    thermocouple_1: i16,  // unit: deg celcius / (3)
    thermocouple_2: i16,  // unit: deg celcius / (3)
    thermocouple_3: i16,  // unit: deg celcius / (3)
    thermocouple_4: i16,  // unit: deg celcius / (3)
    thermocouple_5: i16,  // unit: deg celcius / (3)
    thermocouple_6: i16,  // unit: deg celcius / (3)
    thermocouple_7: i16,  // unit: deg celcius / (3)
    thermocouple_8: i16,  // unit: deg celcius / (3)
    thermocouple_9: i16,  // unit: deg celcius / (3)
    thermocouple_10: i16,  // unit: deg celcius / (3)
    thermocouple_11: i16,  // unit: deg celcius / (3)
    thermocouple_12: i16,  // unit: deg celcius / (3)
    thermocouple_13: i16,  // unit: deg celcius / (3)
    thermocouple_14: i16,  // unit: deg celcius / (3)
    gp_input_1: [char; 6],  // unit: variable / n * 10 / +- value unit / +1234C (6)
    gp_input_2: [char; 6],  // unit: variable / n * 10 / +- value unit / +1234C (6)
    gp_input_3: [char; 6],  // unit: variable / n * 10 / +- value unit / +1234C (6)
    gp_input_4: [char; 6],  // unit: variable / n * 10 / +- value unit / +1234C (6)
    gp_input_5: [char; 6],  // unit: variable / n * 10 / +- value unit / +1234C (6)
    gp_input_6: [char; 6],  // unit: variable / n * 10 / +- value unit / +1234C (6)
    gp_input_7: [char; 6],  // unit: variable / n * 10 / +- value unit / +1234C (6)
    gp_input_8: [char; 6],  // unit: variable / n * 10 / +- value unit / +1234C (6)
    gp_input_9: [char; 6],  // unit: variable / n * 10 / +- value unit / +1234C (6)
    gp_input_10: [char; 6],  // unit: variable / n * 10 / +- value unit / +1234C (6)
    gp_input_11: [char; 6],  // unit: variable / n * 10 / +- value unit / +1234C (6)
    gp_input_12: [char; 6],  // unit: variable / n * 10 / +- value unit / +1234C (6)
    gp_input_13: [char; 6],  // unit: variable / n * 10 / +- value unit / +1234C (6)
    contacts: [char; 16],  // Not in use / Z*16(16)

    power_percent: i16,  // 0-200 / unit: % / XXX
    egt_leaning_status: char  // L=lean of peak P=peak R:rich of peak X: not available / (1)
}
*/

#[derive(Debug)]
pub struct D1x0EMSData {
    bytes: D1x0EmsFrame,
    frame_start: usize,
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


impl D1x0EMSData {
    pub fn new(time_count: u32, freq: Option<u32>) -> Self {
        let f = freq.unwrap_or(100);
        let phase: f32 = (( time_count as f32 % f as f32 ) * 180.0 / f as f32).to_radians();
        //println!("Phase: {}", phase);
        Self {
            bytes: [b'0'; D1X0_EMS_FRAME_LENGTH],
            frame_start: 0,
            system_time: time_count,
            manifold_pressure: 0.0,
            oil_temp: 0,
            oil_pressure: (50.0 * f32::sin(phase)) as u16,
            fuel_pressure: 0.0,
            volts: 12.0,
            amps: 5.0,
            rpm: (3000.0 * f32::sin(phase)) as u16,
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
        // system_time
        let time_string = format!("{:0>8}", self.system_time.to_string());
        self.bytes[..SYSTEM_TIME_LENGTH].copy_from_slice(time_string.as_bytes());

        // oil pressure
        let oil_pressure = format!("{:0>3}", self.oil_pressure.to_string());
        self.bytes[15 .. 18].copy_from_slice(oil_pressure.as_bytes());

        // rpm
        let rpm = format!("{:0>3}", (self.rpm / 10).to_string());
        self.bytes[27 .. 30].copy_from_slice(rpm.as_bytes());

        // GP1-3 as unused 'XXXXXXXX'
        self.bytes[43 .. 67].copy_from_slice("XXXXXXXXXXXXXXXXXXXXXXXX".as_bytes());

        // crc
        let crc = Self::calc_crc(&self.bytes);

        Self::add_crc_crlf(crc, &mut self.bytes);
    }
}

impl DynonSerialize for D1x0EMSData {

    fn as_bytes(&mut self) -> &[u8] {

        if (self.frame_start == 0) {
            self.update();
        }

        let mut rng = rng();
        let slice_start = self.frame_start;
        let slice_end = rng.random_range((slice_start+1)..D1X0_EMS_FRAME_LENGTH+1);
        self.frame_start = slice_end;
        if self.frame_start >= D1X0_EMS_FRAME_LENGTH {
            self.frame_start = 0;
        }
        &self.bytes[slice_start..slice_end]
    }
}

//---------------------------

#[derive(Debug)]
pub struct D1x0EFISData {
    bytes: D1x0EfisFrame,
    frame_start: usize,
    system_time: u32,  // HHMMSSFF
    pitch: f32,  // unit: deg / n * 10 / positive = pitch up / XXXX is unavailable
    roll: f32,  // unit: deg / n * 10 / positive = right bank / XXXXX
    yaw: u16,  // unit: deg / XXX
    ias: f32,  // IAS / unit: m/s / n * 10 / XXXX
//    pressure_alt: i16,  // unit : feet / at 29.92Hg / can be minus (sea level) / XXXXXX
//    displayed_alt: i16,
//    turn_rate: f32,  // unit: deg / XXXX
//    vertical_speed: f32,  // unit: ft/min / positive = climbing / XXX(4)
    lateral_accel: f32,  // unit: g / n * 100 / positive = left skid (ball right) / XXX
    vertical_accel: f32,  // unit: g / n * 100 / positive = going up accel / XXX
    aoa: u16,  // AOA / unit: % / XX
}


impl D1x0EFISData {
    pub fn new(time_count: u32, freq: Option<u32>) -> Self {
        let f = freq.unwrap_or(100);
        let phase: f32 = (( time_count as f32 % f as f32 ) * 360.0 / f as f32).to_radians();
        //println!("Phase: {}", phase);
        Self {
            bytes: [b'0'; D1X0_EFIS_FRAME_LENGTH],
            frame_start: 0,
            system_time: time_count,
            pitch: (25.0 * f32::sin(phase)),
            roll: (60.0 * f32::sin(phase)),
            yaw: (90.0 * f32::sin(phase)) as u16 + 180,
            ias: 0.0,
            lateral_accel: 0.0,
            vertical_accel: 1.0,
            aoa: 0
        }
    }

    fn update(&mut self) {
        // system_time
        let time_string = format!("{:0>8}", self.system_time.to_string());
        self.bytes[..SYSTEM_TIME_LENGTH].copy_from_slice(time_string.as_bytes());

        // pitch
        self.bytes[8] = if self.pitch < 0.0  { b'-' } else { b'+' };
        let pitch = format!("{:0>3}", ((self.pitch * 10.0).abs() as u32).to_string());
        //println!("Pitch {:?}", pitch.as_bytes());
        self.bytes[9 .. 12].copy_from_slice(pitch.as_bytes());

        // roll
        self.bytes[12] = if self.roll < 0.0  { b'-' } else { b'+' };
        let roll = format!("{:0>4}", ((self.roll * 10.0).abs() as u32).to_string());
        self.bytes[13 .. 17].copy_from_slice(roll.as_bytes());

        // // yaw
        // let yaw = format!("{:0>3}", self.yaw.to_string());
        // data[18 .. 21].copy_from_slice(roll.as_bytes());


        // crc
        let crc = Self::calc_crc(&self.bytes);

        Self::add_crc_crlf(crc, &mut self.bytes);
    }
}


impl DynonSerialize for D1x0EFISData {

    fn as_bytes(&mut self) -> &[u8] {

        if (self.frame_start == 0) {
            self.update();
        }

        let mut rng = rng();
        let slice_start = self.frame_start;
        let slice_end = rng.random_range((slice_start+1)..D1X0_EMS_FRAME_LENGTH+1);
        self.frame_start = slice_end;
        if self.frame_start >= D1X0_EMS_FRAME_LENGTH {
            self.frame_start = 0;
        }
        &self.bytes[slice_start..slice_end]
    }
}

//--------------------------------

#[test]
fn test_d1x0_efis_calc_crc() {
    let mut x: D1x0EfisFrame = [0; D1X0_EFIS_FRAME_LENGTH];
    x.copy_from_slice("00082119+058-00541301200+9141+011-01+15003EA0C701A400".as_bytes());
     println!("X: {:?}", x);
    let y = D1x0EFISData::calc_crc(&x);
    //println!("Y: {:?}",y);
    assert_eq!(y,0xA4);
}


#[test]
fn test_d1x0_ems_calc_crc() {
    let mut x: D1x0EmsFrame = [0; D1X0_EMS_FRAME_LENGTH];
    x.copy_from_slice("0012224826351340262441240122631320562191191OAT00090TRE-0061FLP0001020481378139214061421143514503583533633743843951103D200".as_bytes());

    let y = D1x0EMSData::calc_crc(&x);
    //println!("Y: {:?}",y);
    assert_eq!(y,0xD2);
}


#[test]
fn test_d1x0_efis_serialize() {
    let x = D1x0EFISData::new(12345678, None);
    println!("X: {:?}", x);
    let d = x.serialize();

    match d {
        DynonSerializedData::D1x0Efis(y) => {
            println!("Y: {:?}",str::from_utf8(&y));
            assert_eq!(&y[..8], "12345678".as_bytes());
            let l = y.len();
            assert_eq!(&y[l-4 .. l-2], "6F".as_bytes());
        }
        _ => panic!("Returned wrong type."),
    }
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
