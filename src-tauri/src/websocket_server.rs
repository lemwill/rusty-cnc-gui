pub mod websocket_server {
    use std::net::TcpListener;
    use std::sync::mpsc::{Receiver, Sender};
    use std::thread;
    use tungstenite::protocol::Message;

    pub fn start_websocket_server(tx_ws: Sender<Vec<u8>>, rx_usb: Receiver<Vec<u8>>) {
        let server = TcpListener::bind("127.0.0.1:8081").expect("Failed to bind to address");
        if let Ok(stream) = server.accept() {
            thread::spawn(move || {
                let mut websocket = match tungstenite::accept(stream.0) {
                    Ok(ws) => {
                        println!("New WebSocket connection");
                        ws
                    }
                    Err(e) => {
                        println!("Error during WebSocket handshake: {}", e);
                        return;
                    }
                };

                println!("WebSocket connection established");

                loop {
                    // Log
                    println!("Waiting for WebSocket message");

                    match websocket.read_message() {
                        Ok(msg) => {
                            println!("WebSocket message received");
                            if msg.is_text() || msg.is_binary() {
                                tx_ws.send(msg.into_data()).expect("Failed to send message");
                            }
                        }
                        Err(e) => {
                            println!("Error reading WebSocket message: {}", e);
                            break;
                        }
                    };
                    println!("WebSocket message received");
                    // Sending USB read data to the WebSocket
                    match rx_usb.try_recv() {
                        Ok(data) => {
                            if let Err(e) = websocket.write_message(Message::binary(data)) {
                                println!("Error sending WebSocket message: {}", e);
                                break;
                            }
                        }
                        Err(_) => (),
                    }
                }
            });
        }
    }
}
