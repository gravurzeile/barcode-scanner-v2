// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    barcode_scanner_ui_lib::run();
    barcode_scanner_ui_lib::looper().unwrap();
}
