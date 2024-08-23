
struct ADAHRSData {
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


struct D1x0EMSData {
    system_time: u32,  // HHMMSSFF
    manifold_pressure: f32,  // unit: inHg * 100 (4)
    oil_temp: i16,  // unit : deg celcius / XXX (3)
    oil_pressure: i16,  // unit : psi / (3)
    fuel_pressure: f32,  // unit: PSI / n * 10 / XXX (3)
    volts: f32,  // n * 10 / (3)
    amps: f32,  // n * 10 / (3)
    rpm: i16,  // n / 10 (3)
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
    contact_1: i16, //(1)
    contact_2: i16, //(2)
}


struct D1x0EFISData {
    system_time: u32,  // HHMMSSFF
    pitch: f32,  // unit: deg / n * 10 / positive = pitch up / XXXX is unavailable
    roll: f32,  // unit: deg / n * 10 / positive = right bank / XXXXX
    yaw: i16,  // unit: deg / XXX
    ias: f32,  // IAS / unit: m/s / n * 10 / XXXX
//    pressure_alt: i16,  // unit : feet / at 29.92Hg / can be minus (sea level) / XXXXXX
//    displayed_alt: i16,
//    turn_rate: f32,  // unit: deg / XXXX
//    vertical_speed: f32,  // unit: ft/min / positive = climbing / XXX(4)
    lateral_accel: f32,  // unit: g / n * 100 / positive = left skid (ball right) / XXX
    vertical_accel: f32,  // unit: g / n * 100 / positive = going up accel / XXX
    aoa: i16,  // AOA / unit: % / XX
}