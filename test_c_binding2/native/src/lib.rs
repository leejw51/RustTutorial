mod line;

#[repr(u8)]
pub enum Ammo {
    Rock,
    WaterBalloon,
    Cow,
}

#[repr(C)]
pub struct Target {
    latitude: f64,
    longitude: String,
}


#[no_mangle] pub extern "C" fn process(arg: Target) {
}


#[no_mangle] pub extern "C" fn process(arg: Ammo) {
}



