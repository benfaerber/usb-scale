use usb_scale::{
    Scale,
    ScaleReading,
    ScaleStatus,
    error::{ScaleError, ScaleResult},
    weight::{Weight, WeightUnit},
    vendors::FAIRBANKS_SCB_900_SCALE
};

use std::thread;
use std::time::Duration;

fn main() {
    let mut scale_connection = Scale::connect(FAIRBANKS_SCB_900_SCALE);

    loop {
        scale_connection = match scale_connection {
            Ok(scale) => {
                match scale.read() {
                    Ok(reading) => {
                        println!("Reading: {}", reading);
                        // Status: Is the scale Stable, In Motion, Requires Taring, etc
                        println!("Scale Status: {}", reading.status);
                        println!("Report ID: {}", reading.report_id);
                        println!("Data Scaling: {}", reading.data_scaling);
                        println!("Scale Unit: {:?}", reading.weight.and_then(|w| Some(w.unit)));

                        let scale_weight_text = match reading.weight {
                            Some(weight) => weight.to_string(),
                            None => "No Reading".to_string(),
                        };
                        println!("Scale Weight: {}", scale_weight_text);
                        Ok(scale)
                    },
                    Err(err) => {
                        println!("Could not read scale! {}", err);
                        println!("Attempting to reconnect...");
                        scale.reconnect()
                    }
                }
            },
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