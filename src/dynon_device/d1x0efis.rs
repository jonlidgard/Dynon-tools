#![allow(unused)]
#[cfg(test)]

extern crate rand;
use crate::dynon_device::*;

const D1X0_EFIS_FRAME_LENGTH: usize = 53;
type D1x0EfisFrame = [u8; D1X0_EFIS_FRAME_LENGTH];

#[derive(Debug)]
pub struct D1x0EFISDevice {
    bytes: D1x0EfisFrame,
    frame_start: usize,
    freq: u32,
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


impl D1x0EFISDevice {
    pub fn new(freq: Option<u32>) -> Self {
        Self {
            bytes: [b'0'; D1X0_EFIS_FRAME_LENGTH],
            frame_start: rng().random_range(0..D1X0_EFIS_FRAME_LENGTH),
            freq: freq.unwrap_or(100),
            system_time: 1,
            pitch: 0.0,
            roll: 0.0,
            yaw: 0,
            ias: 0.0,
            lateral_accel: 0.0,
            vertical_accel: 1.0,
            aoa: 0
        }
    }

    fn update(&mut self) {
        let f = self.freq;
        let phase: f32 = (( self.system_time as f32 % f as f32 ) * 180.0 / f as f32).to_radians();
        // system_time
        self.system_time += 1;
        let time_string = format!("{:0>8}", self.system_time.to_string());
        self.bytes[..SYSTEM_TIME_LENGTH].copy_from_slice(time_string.as_bytes());

        // pitch
        self.pitch = (25.0 * f32::sin(phase));
        self.bytes[8] = if self.pitch < 0.0  { b'-' } else { b'+' };
        let pitch = format!("{:0>3}", ((self.pitch * 10.0).abs() as u32).to_string());
        //println!("Pitch {:?}", pitch.as_bytes());
        self.bytes[9 .. 12].copy_from_slice(pitch.as_bytes());

        // roll
        self.roll = (60.0 * f32::sin(phase));
        self.bytes[12] = if self.roll < 0.0  { b'-' } else { b'+' };
        let roll = format!("{:0>4}", ((self.roll * 10.0).abs() as u32).to_string());
        self.bytes[13 .. 17].copy_from_slice(roll.as_bytes());

        // // yaw
        self.yaw = (90.0 * f32::sin(phase)) as u16 + 180;
        // let yaw = format!("{:0>3}", self.yaw.to_string());
        // data[18 .. 21].copy_from_slice(roll.as_bytes());


        // crc
        let crc = Self::calc_crc(&self.bytes);

        Self::add_crc_crlf(crc, &mut self.bytes);
        self.system_time += 1;
    }
}


impl DynonDevice for D1x0EFISDevice {

    fn as_bytes(&mut self) -> (bool, &[u8]) {
        let slice_start = self.frame_start;
        let slice_end = rng().random_range((slice_start+1)..D1X0_EFIS_FRAME_LENGTH+1);
        self.frame_start = slice_end;
        if self.frame_start >= D1X0_EFIS_FRAME_LENGTH {
            self.frame_start = 0;
            self.update();
        }
        (self.frame_start == 0, &self.bytes[slice_start..slice_end])
    }
}

//--------------------------------

#[test]
fn test_d1x0_efis_calc_crc() {
    let mut x: D1x0EfisFrame = [0; D1X0_EFIS_FRAME_LENGTH];
    x.copy_from_slice("00082119+058-00541301200+9141+011-01+15003EA0C701A400".as_bytes());
     println!("X: {:?}", x);
    let y = D1x0EFISDevice::calc_crc(&x);
    //println!("Y: {:?}",y);
    assert_eq!(y,0xA4);
}

#[test]
fn test_d1x0_efis_serialize() {
    let x = D1x0EFISDevice::new(12345678, None);
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
