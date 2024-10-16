use evdev::{Device, InputEventKind};
use std::io::Result;
fn main() -> Result<()> {
    let device_name = "Lixin BCST-60 Keyboard";

    // Durchsuche die verfügbaren Eingabegeräte aus /dev/input/event*
    let event_list = glob::glob("/dev/input/event*").map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    let mut device = event_list
        .filter_map(|entry| {
            let path = entry.ok()?;
            let device = Device::open(&path).ok()?;
            if device.name().unwrap_or("Unknown device").contains(device_name) {
                return Some(device);
            }
            None
        })
        .next()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "No device found"))?;

    // Gerät exklusiv greifen
    device.grab()?;

    // print all infos about the device
    println!("Gerät: {}", device.name().unwrap_or("Unknown device"));

    let mut left_ctrl_key = false;
    let mut home_key = false;
    let mut enter_key = false;
    let mut tempkey = String::new();
    let mut barcode = String::new();

    // Endlosschleife, um kontinuierlich neue Events zu lesen
    loop {
        let events = device.fetch_events().unwrap();

        events.for_each(|ev| {
            if let InputEventKind::Key(key) = ev.kind() {
                if ev.value() == 1 {
                    let newkey = format!("{:?}", key).replace("KEY_", "");

                    println!("newkey: {}", newkey);

                    if newkey == "LEFTCTRL" {
                        left_ctrl_key = true;
                        home_key = false;
                        enter_key = false;
                        tempkey.clear();
                        barcode.clear();
                    }

                    if left_ctrl_key && newkey == "HOME" {
                        home_key = true;
                    }

                    if home_key && newkey == "ENTER" {
                        enter_key = true;
                        barcode = tempkey.clone();
                    }

                    if home_key && !enter_key {
                        if newkey.len() == 1 {
                            tempkey.push_str(&newkey);
                        }
                    }
                }
            }
        });

        println!("tempkey: {}", tempkey);
        println!("barcode: {}", barcode);
    }
}