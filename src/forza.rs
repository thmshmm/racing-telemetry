use super::utils;

pub const MESSAGE_SIZE: usize = 323;

// Source: https://forums.forzamotorsport.net/turn10_postst128499_Forza-Motorsport-7--Data-Out--feature-details.aspx?=
pub struct TelemetryData {
    pub is_race_on: i32, // = 1 when race is on. = 0 when in menus/race stopped

    pub timestamp_ms: u32, // Can overflow to 0 eventually

    pub engine_max_rpm: f32,
    pub engine_idle_rpm: f32,
    pub current_engine_rpm: f32,

    pub acceleration_x: f32, // In the car's local space; X = right, Y = up, Z = forward
    pub acceleration_y: f32,
    pub acceleration_z: f32,

    pub velocity_x: f32, // In the car's local space; X = right, Y = up, Z = forward
    pub velocity_y: f32,
    pub velocity_z: f32,

    pub angular_velocity_x: f32, // In the car's local space; X = pitch, Y = yaw, Z = roll
    pub angular_velocity_y: f32,
    pub angular_velocity_z: f32,

    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,

    pub normalized_suspension_travel_front_left: f32, // Suspension travel normalized: 0.0f = max stretch; 1.0 = max compression
    pub normalized_suspension_travel_front_right: f32,
    pub normalized_suspension_travel_rear_left: f32,
    pub normalized_suspension_travel_rear_right: f32,

    pub tire_slip_ratio_front_left: f32, // Tire normalized slip ratio, = 0 means 100% grip and |ratio| > 1.0 means loss of grip
    pub tire_slip_ratio_front_right: f32,
    pub tire_slip_ratio_rear_left: f32,
    pub tire_slip_ratio_rear_right: f32,

    pub wheel_rotation_speed_front_left: f32, // Wheel rotation speed radians/sec
    pub wheel_rotation_speed_front_right: f32,
    pub wheel_rotation_speed_rear_left: f32,
    pub wheel_rotation_speed_rear_right: f32,

    pub wheel_on_rumble_strip_front_left: i32, // = 1 when wheel is on rumble strip, = 0 when off
    pub wheel_on_rumble_strip_front_right: i32,
    pub wheel_on_rumble_strip_rear_left: i32,
    pub wheel_on_rumble_strip_rear_right: i32,

    pub wheel_in_puddle_depth_front_left: f32, // = from 0 to 1, where 1 is the deepest puddle
    pub wheel_in_puddle_depth_front_right: f32,
    pub wheel_in_puddle_depth_rear_left: f32,
    pub wheel_in_puddle_depth_rear_right: f32,

    pub surface_rumble_front_left: f32, // Non-dimensional surface rumble values passed to controller force feedback
    pub surface_rumble_front_right: f32,
    pub surface_rumble_rear_left: f32,
    pub surface_rumble_rear_right: f32,

    pub trire_slip_angle_front_left: f32, // Tire normalized slip angle, = 0 means 100% grip and |angle| > 1.0 means loss of grip
    pub trire_slip_angle_front_right: f32,
    pub trire_slip_angle_rear_left: f32,
    pub trire_slip_angle_rear_right: f32,

    pub tire_combined_slip_front_left: f32, // Tire normalized combined slip, = 0 means 100% grip and |slip| > 1.0 means loss of grip
    pub tire_combined_slip_front_right: f32,
    pub tire_combined_slip_rear_left: f32,
    pub tire_combined_slip_rear_right: f32,

    pub suspension_travel_meters_front_left: f32, // Actual suspension travel in meters
    pub suspension_travel_meters_front_right: f32,
    pub suspension_travel_meters_rear_left: f32,
    pub suspension_travel_meters_rear_right: f32,

    pub car_ordinal: i32,           // Unique ID of the car make/model
    pub car_class: i32, // Between 0 (D -- worst cars) and 7 (X class -- best cars) inclusive
    pub car_performance_index: i32, // Between 100 (slowest car) and 999 (fastest car) inclusive

    pub drivetrain_type: i32, // Corresponds to EDrivetrainType; 0 = FWD, 1 = RWD, 2 = AWD
    pub num_cylinders: i32,

    pub position_x: f32, // Position (meters)
    pub position_y: f32,
    pub position_z: f32,

    pub speed: f32,  // meters per second
    pub power: f32,  // Watts
    pub torque: f32, // Newton meter

    pub tire_temp_front_left: f32,
    pub tire_temp_front_right: f32,
    pub tire_temp_rear_left: f32,
    pub tire_temp_rear_right: f32,

    pub boost: f32,
    pub fuel: f32,
    pub distance_traveled: f32,

    pub best_lap: f32,
    pub last_lap: f32,
    pub current_lap: f32,
    pub current_race_time: f32,

    pub lap_number: u16,
    pub race_position: u8,

    pub accel: u8,
    pub brake: u8,
    pub clutch: u8,
    pub hand_brake: u8,
    pub gear: u8,
    pub steer: i8,

    pub normalized_driving_line: i8,
    pub normalized_ai_brake_difference: i8,
}

pub fn parse(buffer: &[u8]) -> TelemetryData {
    return TelemetryData {
        is_race_on: i32::from_le_bytes(utils::get32(&buffer[0..4])),
        timestamp_ms: u32::from_le_bytes(utils::get32(&buffer[4..8])),
        engine_max_rpm: f32::from_le_bytes(utils::get32(&buffer[8..12])),
        engine_idle_rpm: f32::from_le_bytes(utils::get32(&buffer[12..16])),
        current_engine_rpm: f32::from_le_bytes(utils::get32(&buffer[16..20])),
        acceleration_x: f32::from_le_bytes(utils::get32(&buffer[20..24])),
        acceleration_y: f32::from_le_bytes(utils::get32(&buffer[24..28])),
        acceleration_z: f32::from_le_bytes(utils::get32(&buffer[28..32])),
        velocity_x: f32::from_le_bytes(utils::get32(&buffer[32..36])),
        velocity_y: f32::from_le_bytes(utils::get32(&buffer[36..40])),
        velocity_z: f32::from_le_bytes(utils::get32(&buffer[40..44])),
        angular_velocity_x: f32::from_le_bytes(utils::get32(&buffer[44..48])),
        angular_velocity_y: f32::from_le_bytes(utils::get32(&buffer[48..52])),
        angular_velocity_z: f32::from_le_bytes(utils::get32(&buffer[52..56])),
        yaw: f32::from_le_bytes(utils::get32(&buffer[56..60])),
        pitch: f32::from_le_bytes(utils::get32(&buffer[60..64])),
        roll: f32::from_le_bytes(utils::get32(&buffer[64..68])),
        normalized_suspension_travel_front_left: f32::from_le_bytes(utils::get32(&buffer[68..72])),
        normalized_suspension_travel_front_right: f32::from_le_bytes(utils::get32(&buffer[72..76])),
        normalized_suspension_travel_rear_left: f32::from_le_bytes(utils::get32(&buffer[76..80])),
        normalized_suspension_travel_rear_right: f32::from_le_bytes(utils::get32(&buffer[80..84])),
        tire_slip_ratio_front_left: f32::from_le_bytes(utils::get32(&buffer[84..88])),
        tire_slip_ratio_front_right: f32::from_le_bytes(utils::get32(&buffer[88..92])),
        tire_slip_ratio_rear_left: f32::from_le_bytes(utils::get32(&buffer[92..96])),
        tire_slip_ratio_rear_right: f32::from_le_bytes(utils::get32(&buffer[96..100])),
        wheel_rotation_speed_front_left: f32::from_le_bytes(utils::get32(&buffer[100..104])),
        wheel_rotation_speed_front_right: f32::from_le_bytes(utils::get32(&buffer[104..108])),
        wheel_rotation_speed_rear_left: f32::from_le_bytes(utils::get32(&buffer[108..112])),
        wheel_rotation_speed_rear_right: f32::from_le_bytes(utils::get32(&buffer[112..116])),
        wheel_on_rumble_strip_front_left: i32::from_le_bytes(utils::get32(&buffer[116..120])),
        wheel_on_rumble_strip_front_right: i32::from_le_bytes(utils::get32(&buffer[120..124])),
        wheel_on_rumble_strip_rear_left: i32::from_le_bytes(utils::get32(&buffer[124..128])),
        wheel_on_rumble_strip_rear_right: i32::from_le_bytes(utils::get32(&buffer[128..132])),
        wheel_in_puddle_depth_front_left: f32::from_le_bytes(utils::get32(&buffer[132..136])),
        wheel_in_puddle_depth_front_right: f32::from_le_bytes(utils::get32(&buffer[136..140])),
        wheel_in_puddle_depth_rear_left: f32::from_le_bytes(utils::get32(&buffer[140..144])),
        wheel_in_puddle_depth_rear_right: f32::from_le_bytes(utils::get32(&buffer[144..148])),
        surface_rumble_front_left: f32::from_le_bytes(utils::get32(&buffer[148..152])),
        surface_rumble_front_right: f32::from_le_bytes(utils::get32(&buffer[152..156])),
        surface_rumble_rear_left: f32::from_le_bytes(utils::get32(&buffer[156..160])),
        surface_rumble_rear_right: f32::from_le_bytes(utils::get32(&buffer[160..164])),
        trire_slip_angle_front_left: f32::from_le_bytes(utils::get32(&buffer[164..168])),
        trire_slip_angle_front_right: f32::from_le_bytes(utils::get32(&buffer[168..172])),
        trire_slip_angle_rear_left: f32::from_le_bytes(utils::get32(&buffer[172..176])),
        trire_slip_angle_rear_right: f32::from_le_bytes(utils::get32(&buffer[176..180])),
        tire_combined_slip_front_left: f32::from_le_bytes(utils::get32(&buffer[180..184])),
        tire_combined_slip_front_right: f32::from_le_bytes(utils::get32(&buffer[184..188])),
        tire_combined_slip_rear_left: f32::from_le_bytes(utils::get32(&buffer[188..192])),
        tire_combined_slip_rear_right: f32::from_le_bytes(utils::get32(&buffer[192..196])),
        suspension_travel_meters_front_left: f32::from_le_bytes(utils::get32(&buffer[196..200])),
        suspension_travel_meters_front_right: f32::from_le_bytes(utils::get32(&buffer[200..204])),
        suspension_travel_meters_rear_left: f32::from_le_bytes(utils::get32(&buffer[204..208])),
        suspension_travel_meters_rear_right: f32::from_le_bytes(utils::get32(&buffer[208..212])),
        car_ordinal: i32::from_le_bytes(utils::get32(&buffer[212..216])),
        car_class: i32::from_le_bytes(utils::get32(&buffer[216..220])),
        car_performance_index: i32::from_le_bytes(utils::get32(&buffer[220..224])),
        drivetrain_type: i32::from_le_bytes(utils::get32(&buffer[224..228])),
        num_cylinders: i32::from_le_bytes(utils::get32(&buffer[228..232])),
        position_x: f32::from_le_bytes(utils::get32(&buffer[244..248])),
        position_y: f32::from_le_bytes(utils::get32(&buffer[248..252])),
        position_z: f32::from_le_bytes(utils::get32(&buffer[252..256])),
        speed: f32::from_le_bytes(utils::get32(&buffer[256..260])),
        power: f32::from_le_bytes(utils::get32(&buffer[260..264])),
        torque: f32::from_le_bytes(utils::get32(&buffer[264..268])),
        tire_temp_front_left: f32::from_le_bytes(utils::get32(&buffer[268..272])),
        tire_temp_front_right: f32::from_le_bytes(utils::get32(&buffer[272..276])),
        tire_temp_rear_left: f32::from_le_bytes(utils::get32(&buffer[276..280])),
        tire_temp_rear_right: f32::from_le_bytes(utils::get32(&buffer[280..284])),
        boost: f32::from_le_bytes(utils::get32(&buffer[284..288])),
        fuel: f32::from_le_bytes(utils::get32(&buffer[288..292])),
        distance_traveled: f32::from_le_bytes(utils::get32(&buffer[292..296])),
        best_lap: f32::from_le_bytes(utils::get32(&buffer[296..300])),
        last_lap: f32::from_le_bytes(utils::get32(&buffer[300..304])),
        current_lap: f32::from_le_bytes(utils::get32(&buffer[304..308])),
        current_race_time: f32::from_le_bytes(utils::get32(&buffer[308..312])),
        lap_number: u16::from_le_bytes(utils::get16(&buffer[312..314])),
        race_position: u8::from_le_bytes(utils::get8(&buffer[314..315])),
        accel: u8::from_le_bytes(utils::get8(&buffer[315..316])),
        brake: u8::from_le_bytes(utils::get8(&buffer[316..317])),
        clutch: u8::from_le_bytes(utils::get8(&buffer[317..318])),
        hand_brake: u8::from_le_bytes(utils::get8(&buffer[318..319])),
        gear: u8::from_le_bytes(utils::get8(&buffer[319..320])),
        steer: i8::from_le_bytes(utils::get8(&buffer[320..321])),
        normalized_driving_line: i8::from_le_bytes(utils::get8(&buffer[321..322])),
        normalized_ai_brake_difference: i8::from_le_bytes(utils::get8(&buffer[322..323])),
    };
}
