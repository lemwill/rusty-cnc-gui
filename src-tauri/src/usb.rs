use crate::common::{Channels, Data};
use rusb::UsbContext;
use tokio::sync::mpsc::UnboundedReceiver;
pub async fn handle_usb(
    channels: Channels,
    mut usb_rx: UnboundedReceiver<Data>,
    vendor_id: u16,
    product_id: u16,
) -> rusb::Result<()> {
    let context = rusb::Context::new()?;

    loop {
        // Specify the device you want to connect to
        let mut device = match context.open_device_with_vid_pid(vendor_id, product_id) {
            Some(device) => device,
            None => {
                println!("Could not find USB device. Retrying...");
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                continue;
            }
        };

        println!("Connected to device");

        let interface = 1;

        if let Err(e) = device.claim_interface(interface) {
            println!("Failed to claim interface: {}", e);
            continue;
        }
        println!("Interface {} claimed", interface);

        loop {
            // Use select! to concurrently handle USB data and read from device
            tokio::select! {
                Some(data) = usb_rx.recv() => {
                    // Send the data to the device
                    println!("Sending data to device {:?}", data);
                    match device.write_interrupt(1, &data.data, std::time::Duration::from_millis(1)) {
                        Ok(_) => println!("Sent data to device"),
                        Err(_) => {
                            println!("Failed to write to device. Attempting to reconnect...");
                            break;
                        },
                    }
                },
                _ = tokio::time::sleep(std::time::Duration::from_millis(5)) => {
                    // Read from the device
                    let mut data = vec![0; 1024];
                    match device.read_interrupt(130, &mut data, std::time::Duration::from_millis(25)) {
                        Ok(bytes_read) => {
                            // Send the data to the websocket handler
                            data.truncate(bytes_read); // truncate buffer to actual data size

                            channels.usb_to_ws.send(Data { data: data.clone() });

                            data.resize(1024, 0); // resize buffer back to max size for next read

                            println!("Received data from device");
                        }
                        Err(rusb::Error::Timeout) => {
                            // Timeout is expected, so just continue
                            continue;
                        }
                        Err(_) => {
                            println!("Failed to read from device. Attempting to reconnect...");
                            break;
                        }
                    }
                }
            }
        }
    }
}
