use std::io::Result;

use evdev::{Device, InputEventKind};
use tauri::Emitter;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            let handle = app.handle().clone();
            std::thread::spawn(move || {
                looper(handle).expect("Failed to start looper");
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn loop_device(mut device: Device, handle: tauri::AppHandle) {
    let mut left_ctrl_key = false;
    let mut home_key = false;
    let mut enter_key = false;
    let mut tempkey = String::new();
    let mut barcode = String::new();
    
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
                        handle.emit("barcode-scanned", barcode.clone()).unwrap();
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

fn looper(handle: tauri::AppHandle) -> Result<()> {


    let device_name = "Lixin BCST-60 Keyboard";
    
    // Endlosschleife, um kontinuierlich neue Events zu lesen
    loop {
        // Durchsuche die verfügbaren Eingabegeräte aus /dev/input/event*
        let event_list = glob::glob("/dev/input/event*").map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        let device = event_list
            .filter_map(|entry| {
                let path = entry.ok()?;
                let device = Device::open(&path).ok()?;
                if device.name().unwrap_or("Unknown device").contains(device_name) {
                    return Some(device);
                }
                None
            })
            .next();

        if let Some(mut device) = device {
            // Gerät exklusiv greifen
            device.grab()?;
            // print all infos about the device
            println!("Gerät: {}", device.name().unwrap_or("Unknown device"));
            loop_device(device, handle.clone());
        } else {
            println!("Kein Gerät gefunden, warte auf ein Gerät...");
            std::thread::sleep(std::time::Duration::from_secs(5));
        }
    }
}



