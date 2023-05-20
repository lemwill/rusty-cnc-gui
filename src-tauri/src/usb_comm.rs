use super::*;
use rusb::UsbContext;
use std::collections::VecDeque;
use std::sync::mpsc::{self, TryRecvError, Sender, Receiver};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

pub mod usb_comm {
    use super::*;

    pub struct UsbComm {
        write_tx: Sender<Vec<u8>>,
        read_rx: Receiver<Vec<u8>>,
        stop_signal: std::sync::Arc<AtomicBool>,
        usb_thread: Option<thread::JoinHandle<()>>,
    }

    impl UsbComm {
        pub fn new(vendor_id: u16, product_id: u16) -> UsbComm {
            let (write_tx, write_rx) = mpsc::channel();
            let (read_tx, read_rx) = mpsc::channel();
            let stop_signal = std::sync::Arc::new(AtomicBool::new(false));

            let stop_signal_clone = std::sync::Arc::clone(&stop_signal);
            let usb_thread = thread::spawn(move || {
                handle_usb_communication(write_rx, read_tx, stop_signal_clone, vendor_id, product_id);
            });

            UsbComm {
                write_tx,
                read_rx,
                stop_signal,
                usb_thread: Some(usb_thread),
            }
        }

        pub fn send_data(&self, data: Vec<u8>) {
            self.write_tx.send(data).unwrap();
        }

        pub fn receive_data(&self) -> Option<Vec<u8>> {
            self.read_rx.try_recv().ok()
        }

        pub fn stop(&mut self) {
            self.stop_signal.store(true, Ordering::Relaxed);
            self.usb_thread.take().expect("Thread has already stopped").join().unwrap();
        }
    }

    fn handle_usb_communication(write_rx: Receiver<Vec<u8>>, read_tx: Sender<Vec<u8>>, stop_signal: std::sync::Arc<AtomicBool>, vendor_id: u16, product_id: u16) {
        let context = rusb::Context::new().unwrap();
        let mut write_queue: VecDeque<Vec<u8>> = VecDeque::new();

        // Attempt to connect to the device initially
        let mut device_handle = match find_and_open_device(&context, vendor_id, product_id) {
            Ok(handle) => handle,
            Err(e) => {
                println!("Failed to connect to device: {}", e);
                return; 
            },
        };

        loop {
            if stop_signal.load(Ordering::Relaxed) {
                break;
            }
             
            match write_rx.try_recv() {
                Ok(write_data) => write_queue.push_back(write_data),
                Err(TryRecvError::Empty) => (),
                Err(TryRecvError::Disconnected) => break,
            }

            // If there is data to write, send it
            if let Some(write_data) = write_queue.pop_front() {
                write_to_device(&mut device_handle, &context, &write_data, vendor_id, product_id);
            }

            // Read from the device and send to read channel
            read_from_device(&mut device_handle, &context, &read_tx, vendor_id, product_id);
        }

        match device_handle.release_interface(1) {
            Ok(_) => println!("Interface released successfully"),
            Err(e) => println!("Failed to release interface: {}", e),
        } 
    }

    fn find_and_open_device(
        context: &rusb::Context,
        vendor_id: u16,
        product_id: u16
    ) -> rusb::Result<rusb::DeviceHandle<rusb::Context>> {
        let device = context.devices()?.iter().find(|device| {
            let device_desc = device.device_descriptor().unwrap();
            device_desc.vendor_id() == vendor_id && device_desc.product_id() == product_id
        });
      
        let device = match device {
            Some(device) => device,
            None => {
                // Sleep for a bit to avoid spamming the console
                thread::sleep(Duration::from_millis(1000));
                return Err(rusb::Error::NoDevice);
            } 
        };
     
        let interface = 1; 
        let mut device_handle = device.open()?;
    
        let _ = device_handle.reset();
    
        // Claim the interface
        match device_handle.claim_interface(interface) {
            Ok(_) => println!("Interface {} claimed", interface),
            Err(e) => {
                println!("Failed to claim interface: {}", e);
                // Sleep for a bit to avoid spamming the console
                thread::sleep(Duration::from_millis(1000));
            },
        }
    
        Ok(device_handle)
    }
    
    fn write_to_device(device_handle: &mut rusb::DeviceHandle<rusb::Context>, context: &rusb::Context, write_data: &[u8], vendor_id: u16, product_id: u16) {
        match device_handle.write_interrupt(1, write_data, Duration::from_millis(1)) {
            Ok(bytes_written) => println!("Wrote {} bytes", bytes_written),
            Err(rusb::Error::NoDevice) => {
                println!("Device disconnected during write. Attempting to reconnect...");
      
                // Attempt to reconnect to the device
                *device_handle = match find_and_open_device(context, vendor_id, product_id) {
                    Ok(handle) => handle, 
                    Err(e) => {
                        println!("Failed to reconnect: {}", e);

                        return;
                    } 
                };  
            }
            Err(e) => println!("Write error: {}", e),
        }
    }
    
    fn read_from_device(device_handle: &mut rusb::DeviceHandle<rusb::Context>, context: &rusb::Context, read_tx: &Sender<Vec<u8>>, vendor_id: u16, product_id: u16) {
        let mut buffer = vec![0; 128]; // Adjust buffer size as necessary
        loop { 
            match device_handle.read_interrupt(
                130,    
                &mut buffer,
                std::time::Duration::from_millis(10),
            ) { 
                Ok(bytes_read) => { 
                    println!("Read {} bytes: {:?}", bytes_read, &buffer[..bytes_read]);
                    buffer.truncate(bytes_read); // truncate buffer to actual data size

                    if read_tx.send(buffer.clone()).is_err() {
                        break;
                    }
                    buffer.resize(128, 0);  // resize buffer back to max size for next read

                }
    
                Err(rusb::Error::NoDevice) => {
                    println!("Device disconnected. Attempting to reconnect...");
    
                    // Attempt to reconnect to the device
                    *device_handle = match find_and_open_device(context, vendor_id, product_id) {
                        Ok(handle) => handle,
                        Err(e) => {
                            println!("Failed to reconnect: {}", e);
                            return;
                        }
                    };
                }
                Err(rusb::Error::Timeout) => break,
    
                Err(e) => println!("Read error: {}", e),
            }
        }
    
    }
    
}

