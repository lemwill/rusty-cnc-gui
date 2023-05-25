#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod common;
mod usb;
mod websocket;

use common::{Channels, Data};
use tokio::sync::mpsc::unbounded_channel;
use tokio::task;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send>> {
    let (usb_to_ws_tx, mut usb_to_ws_rx) = unbounded_channel::<Data>();
    let (ws_to_usb_tx, mut ws_to_usb_rx) = unbounded_channel::<Data>();

    let usb_channels = Channels {
        usb_to_ws: usb_to_ws_tx.clone(),
        ws_to_usb: ws_to_usb_tx.clone(),
    };

    let ws_channels = Channels {
        usb_to_ws: usb_to_ws_tx,
        ws_to_usb: ws_to_usb_tx,
    };

    let vendor_id = 0xc0de; // replace with your vendor id
    let product_id = 0xcafe; // replace with your product id

    // Start the USB and WebSocket handlers
    let usb_handle = task::spawn(usb::handle_usb(
        usb_channels,
        ws_to_usb_rx,
        vendor_id,
        product_id,
    ));
    let ws_handle = task::spawn(websocket::handle_websocket(ws_channels, usb_to_ws_rx));

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    // Wait for both handlers to complete
    let _ = tokio::try_join!(usb_handle, ws_handle);

    Ok(())
}
