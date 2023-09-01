# USB Scale [![crates.io](https://img.shields.io/crates/v/usb_scale.svg)](https://crates.io/crates/usb_scale?icon=rust)

Interact with USB Scales with Rust! Uses the `hidapi` crate to interact with USB devices.

## Getting Started
Add to you `Cargo.toml`:
```toml
# Without Serde
usb_scale = { version = "0.1.0", features = [] }

# With Serde
usb_scale = { version = "0.1.0", features = ["serde"] }
```

Connect to the scale:
```rust
fn connect_to_scale() {
    use usb_scale::{Scale, vendors::FAIRBANKS_SCB_900_SCALE};

    match Scale::connect(FAIRBANKS_SCB_900_SCALE) {
        Ok(scale) => println!("{:?}", scale),
        Err(err) => println!("Error: {:?}", err),
    }
}

```

Get a reading:
```rust
fn get_a_reading() {
    use usb_scale::{Scale, ScaleReading, error::ScaleResult, vendors::FAIRBANKS_SCB_900_SCALE};

    let scale = Scale::connect(FAIRBANKS_SCB_900_SCALE).unwrap();
    let reading: ScaleResult<ScaleReading> = scale.read();
    println!("{:?}", reading);
}
```

Use custom vendor info (You can usually find this info in the manual for your scale):
```rust
fn use_custom_vendor_info() {
    use usb_scale::{Scale, vendors::VendorInfo};
    const FAIRBANKS_VENDOR_ID: u16 = 0x0B67;
    const FAIRBANKS_SCB_900_PRODUCT_ID: u16 = 0x555E;

    let fairbanks_scb_900 = VendorInfo::new(FAIRBANKS_VENDOR_ID, FAIRBANKS_SCB_900_PRODUCT_ID);
    let scale = Scale::connect(fairbanks_scb_900).unwrap();
}
```

A more complex example (found in `examples/src/main.rs`):
```rust
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

```
