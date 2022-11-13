pub enum Reserved {
    BeginningOfFrame,
    Escape,
    ResumeTransmission,
    StopTransmission,
    Reserved,
}

impl From<Reserved> for u8 {
    fn from(reserved: Reserved) -> Self {
        reserved.to_u8()
    }
}

impl Reserved {
    const fn to_u8(&self) -> u8 {
        match self {
            Reserved::BeginningOfFrame => 0x7e,
            Reserved::Escape => 0x7d,
            Reserved::ResumeTransmission => 0x11,
            Reserved::StopTransmission => 0x13,
            Reserved::Reserved => 0xf8,
        }
    }

    pub fn is_reserved<T: Into<u8>>(value: T) -> bool {
        RESERVED.contains(&value.into())
    }
}

use Reserved::*;
const RESERVED: [u8; 5] = [
    BeginningOfFrame.to_u8(),
    Escape.to_u8(),
    ResumeTransmission.to_u8(),
    StopTransmission.to_u8(),
    Reserved.to_u8(),
];
