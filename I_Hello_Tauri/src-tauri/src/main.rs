#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::io::{self, Write};
use std::time::Duration;

#[tauri::command]
fn get_ports() -> Vec<String> {
    let mut ports = Vec::new();
    for port in serialport::available_ports().unwrap() {
        for _ in 0..3 {
            ports.push(port.port_name.clone());
        }
    }
    println!("{:?}", ports);
    return ports.into();
}

// Open port and return true if successful or false if not
#[tauri::command]
fn open_port(port_name: String, baudrate: String) -> String {
    println!("{} {}", port_name, baudrate.parse::<u32>().unwrap());

    let parsed_baudrate = baudrate.parse::<u32>().unwrap();

    let port = serialport::new(&port_name, parsed_baudrate)
        .timeout(Duration::from_secs(5))
        .open();

    match port {
        Ok(mut port) => {
            // While reading from the port, print what we read to an auxiliary file
            let mut aux_file =
                std::fs::File::create("aux.txt").expect("Could not create aux.txt");
            let mut buffer = [0; 128];
            loop {
                let count = port.read(&mut buffer).unwrap();
                aux_file.write_all(&buffer[..count]).unwrap();
                aux_file.flush().unwrap();
                if count == 0 {
                    break;
                }
            }
            return "true".to_string();
        }
        Err(e) => {
            println!("Error opening port: {:?}", e);
            return String::from("Error").into();
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_ports, open_port])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
