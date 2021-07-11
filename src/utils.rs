use std::convert::TryInto;

pub fn get8(a: &[u8]) -> [u8; 1] {
    a.try_into()
        .expect("slice with incorrect length, expected 8 bit")
}

pub fn get16(a: &[u8]) -> [u8; 2] {
    a.try_into()
        .expect("slice with incorrect length, expected 16 bit")
}

pub fn get32(a: &[u8]) -> [u8; 4] {
    a.try_into()
        .expect("slice with incorrect length, expected 32 bit")
}
