use std::thread;
use std::time::Duration;

fn connect_to_scale() {
    use usb_scale::{vendors::FAIRBANKS_SCB_900_SCALE, Scale};

    match Scale::connect(FAIRBANKS_SCB_900_SCALE) {
        Ok(scale) => println!("{:?}", scale),
        Err(err) => println!("Error: {:?}", err),
    }
}

fn get_a_reading() {
    use usb_scale::{error::ScaleResult, vendors::FAIRBANKS_SCB_900_SCALE, Scale, ScaleReading};

    let scale = Scale::connect(FAIRBANKS_SCB_900_SCALE).unwrap();
    let reading: ScaleResult<ScaleReading> = scale.read();
    println!("{:?}", reading);
}

fn use_custom_vendor_info() {
    use usb_scale::{vendors::VendorInfo, Scale};
    const FAIRBANKS_VENDOR_ID: u16 = 0x0B67;
    const FAIRBANKS_SCB_900_PRODUCT_ID: u16 = 0x555E;

    let fairbanks_scb_900 = VendorInfo::new(FAIRBANKS_VENDOR_ID, FAIRBANKS_SCB_900_PRODUCT_ID);
    let scale = Scale::connect(fairbanks_scb_900).unwrap();
}

fn main() {
    use usb_scale::{
        error::{ScaleError, ScaleResult},
        vendors::FAIRBANKS_SCB_900_SCALE,
        weight::{Weight, WeightUnit},
        Scale, ScaleReading, ScaleStatus,
    };

    use std::thread;
    use std::time::Duration;

    let mut scale_connection = Scale::connect(FAIRBANKS_SCB_900_SCALE);

    loop {
        scale_connection = match scale_connection {
            Ok(scale) => {
                let reading = scale.read().unwrap();
                println!("Reading: {}", reading);
                // Status: Is the scale Stable, In Motion, Requires Taring, etc
                println!("Scale Status: {}", reading.status);
                println!("Report ID: {}", reading.report_id);
                println!("Data Scaling: {}", reading.data_scaling);
                println!(
                    "Scale Unit: {:?}",
                    reading.weight.and_then(|w| Some(w.unit))
                );

                let scale_weight_text = reading
                    .weight
                    .map(|x| x.to_string())
                    .unwrap_or("No Reading!".into());
                println!("Scale Weight: {}", scale_weight_text);
                Ok(scale)
            }
            Err(err) => {
                println!("Could not connect to scale! {}", err);
                println!("Attempting to reconnect...");
                Scale::connect(FAIRBANKS_SCB_900_SCALE)
            }
        };
        println!("");

        thread::sleep(Duration::from_millis(200));
    }
}
