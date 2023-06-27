use crate::error::{ScaleError, ScaleResult};
use crate::parser::parse_input_stream;
use crate::weight::Weight;
use crate::vendors::VendorInfo;

use std::fmt;
use std::thread;
use std::time::Duration;

use hidapi::{HidApi, HidDevice};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub(crate) const DATA_SIZE: usize = 6;
pub(crate) type ByteBuffer = [u8; DATA_SIZE];

const RECONNECT_ATTEMPTS: usize = 50;

pub struct Scale {
    device: HidDevice,
    vendor_info: VendorInfo,
}

#[derive(Debug, Clone, PartialEq, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ScaleReading {
    pub report_id: u8,
    pub status: ScaleStatus,
    pub data_scaling: u8,
    pub weight: Option<Weight>,
}

impl fmt::Display for ScaleReading {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let weight = match self.weight {
            Some(weight) => weight.to_string(),
            None => "No Reading".to_string(),
        };

        let msg = format!("{} - {}", self.status, weight);
        write!(f, "{}", msg)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum ScaleStatus {
    NotConnected,
    Fault,
    Stable,
    InMotion,
    UnderZero,
    OverWeight,
    RequiresCalibration,
    RequiresTaring,
}

impl ScaleStatus {
    pub fn is_valid(&self) -> bool {
        *self != Self::Fault && !self.needs_calibration()
    }

    pub fn needs_calibration(&self) -> bool {
        matches!(
            self,
            ScaleStatus::UnderZero | ScaleStatus::RequiresCalibration | ScaleStatus::RequiresTaring
        )
    }
}

impl fmt::Display for ScaleStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            Self::NotConnected => "Not Connected",
            Self::Fault => "Fault",
            Self::Stable => "Stable",
            Self::InMotion => "In Motion",
            Self::UnderZero => "Under Zero",
            Self::OverWeight => "Over Weight",
            Self::RequiresCalibration => "Requires Calibration",
            Self::RequiresTaring => "Requires Taring",
        }.to_string();

        write!(f, "{}", message)
    }
}

impl Scale {
    pub fn connect(vendor_info: VendorInfo) -> ScaleResult<Self> {
        let VendorInfo { vendor_id, product_id } = vendor_info;
        match HidApi::new() {
            Ok(hid) => match hid.open(vendor_id, product_id) {
                Ok(device) => Ok(Self { device, vendor_info }),
                Err(err) => Err(ScaleError::ConnectError(err)),
            },
            Err(err) => Err(ScaleError::UsbError(err)),
        }
    }

    pub fn reconnect(&self) -> ScaleResult<Self> {
        Self::connect(self.vendor_info)
    }

    pub fn read(&self) -> ScaleResult<ScaleReading> {
        let raw = self.read_raw();
        raw.and_then(|bytes| parse_input_stream(bytes))
    }

    fn read_raw(&self) -> ScaleResult<ByteBuffer> {
        let mut buf: ByteBuffer = [0u8; DATA_SIZE];
        for _ in 0..RECONNECT_ATTEMPTS {
            let result = self.device.read_timeout(&mut buf[..], 10);
            if let Ok(size) = result {
                if size == DATA_SIZE {
                    return Ok(buf);
                }
            }
            thread::sleep(Duration::from_millis(10));
        }

        Err(ScaleError::ReadError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_scale_data_test() {
        // Requires Taring
        let requires_taring_parsed = parse_input_stream([3, 8, 12, 254, 0, 0]).unwrap();
        let requires_taring_reading = ScaleReading {
            report_id: 3,
            status: ScaleStatus::RequiresTaring,
            data_scaling: 254,
            weight: None,
        };

        assert_eq!(requires_taring_parsed, requires_taring_reading);

        // Stable with zero weight
        let stable_zero_pounds_parsed = parse_input_stream([3, 4, 12, 254, 0, 0]).unwrap();
        let stable_zero_pounds_reading = ScaleReading {
            report_id: 3,
            status: ScaleStatus::Stable,
            data_scaling: 254,
            weight: Some(Weight::from_pounds(0.0)),
        };

        assert_eq!(stable_zero_pounds_parsed, stable_zero_pounds_reading);

        // In Motion 0.35lbs
        let in_motion_pounds_parsed = parse_input_stream([3, 3, 12, 254, 35, 0]).unwrap();
        let in_motion_pounds_reading = ScaleReading {
            report_id: 3,
            status: ScaleStatus::InMotion,
            data_scaling: 254,
            weight: Some(Weight::from_pounds(0.35)),
        };

        assert_eq!(in_motion_pounds_parsed, in_motion_pounds_reading);

        // Stable 0.14kg
        let stable_kilograms_parsed =  parse_input_stream([3, 4, 3, 254, 14, 0]).unwrap();
        let stable_kilograms_reading = ScaleReading {
            report_id: 3,
            status: ScaleStatus::Stable,
            data_scaling: 254,
            weight: Some(Weight::from_kilograms(0.14)),
        };

        assert_eq!(stable_kilograms_parsed, stable_kilograms_reading);
    }
}
