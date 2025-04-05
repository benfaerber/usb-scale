use crate::error::{ScaleError, ScaleResult};

use crate::reader::{ByteBuffer, ScaleReading, ScaleStatus};
use crate::weight::{Weight, WeightUnit};

const UNIT_BYTE_KILOGRAMS: u16 = 3;
const UNIT_BYTE_OUNCES: u16 = 11;
const UNIT_BYTE_POUNDS: u16 = 12;

pub(crate) fn parse_input_stream(bytes: ByteBuffer) -> ScaleResult<ScaleReading> {
    let [report_id, byte_status, byte_unit, data_scaling, byte_weight_lsb, byte_weight_msb] = bytes;
    let status = status_from_byte(byte_status);

    let correct_weight = !matches!(
        status,
        ScaleStatus::UnderZero
            | ScaleStatus::NotConnected
            | ScaleStatus::RequiresCalibration
            | ScaleStatus::RequiresTaring
    );
    let weight = weight_from_bytes(byte_weight_lsb, byte_weight_msb, byte_unit)?;

    Ok(ScaleReading {
        report_id,
        status,
        data_scaling,
        weight: correct_weight.then_some(weight),
    })
}

fn weight_from_bytes(lsb: u8, msb: u8, unit_byte: u8) -> ScaleResult<Weight> {
    let unit = unit_from_byte(unit_byte)?;

    let lsf = lsb as f64;
    let msf = msb as f64;
    let value = (lsf + msf * 256.) / 100.;
    Ok(Weight::from(value, unit))
}

fn status_from_byte(byte: u8) -> ScaleStatus {
    match byte {
        // For some reason, 0, 2 and 4 all mean stable
        0 | 2 | 4 => ScaleStatus::Stable,
        1 => ScaleStatus::Fault,
        3 => ScaleStatus::InMotion,
        5 => ScaleStatus::UnderZero,
        6 => ScaleStatus::OverWeight,
        7 => ScaleStatus::RequiresCalibration,
        8 => ScaleStatus::RequiresTaring,
        _ => ScaleStatus::Fault,
    }
}

fn unit_from_byte(byte: u8) -> ScaleResult<WeightUnit> {
    match byte as u16 {
        UNIT_BYTE_KILOGRAMS => Ok(WeightUnit::Kilograms),
        UNIT_BYTE_OUNCES => Ok(WeightUnit::Ounces),
        UNIT_BYTE_POUNDS => Ok(WeightUnit::Pounds),
        _ => Err(ScaleError::ParseError),
    }
}
