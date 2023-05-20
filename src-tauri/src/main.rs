#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use ctrlc;
use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
mod usb_comm;
use usb_comm::usb_comm::UsbComm;
mod websocket_server;
use crate::websocket_server::websocket_server::start_websocket_server;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let (ws_tx, ws_rx) = mpsc::channel(); // Channel for WebSocket to USB
    let (usb_tx, usb_rx) = mpsc::channel(); // Channel for USB to WebSocket

    let vendor_id = 0xc0de; // replace with your vendor ID
    let product_id = 0xcafe; // replace with your product ID
    let usb_comm = Arc::new(Mutex::new(UsbComm::new(vendor_id, product_id)));

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    let u = Arc::clone(&usb_comm);

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        println!("Caught Ctrl+C, stopping...");
    })
    .expect("Error setting Ctrl-C handler");

    println!("Running... Press Ctrl+C to stop.");

    let ws_server_thread = thread::spawn(move || {
        let _ = start_websocket_server(ws_tx, usb_rx);
        while running.load(Ordering::SeqCst) {
            if let Ok(data) = ws_rx.try_recv() {
                let mut usb_comm = usb_comm.lock().unwrap();
                usb_comm.send_data(data);
            }

            let mut usb_comm = usb_comm.lock().unwrap();
            if let Some(data) = usb_comm.receive_data() {
                usb_tx.send(data).unwrap();
            }
        }

        println!("Gracefully shut down");
        let mut usb_comm = u.lock().unwrap();
        usb_comm.stop();
    });

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    ws_server_thread.join().unwrap();

    Ok(())
}

/*#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
use log::*;

// Import websocket.rs
mod websocket;
use websocket::start_websocket_server;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    // Print hello world to console
}
*/
