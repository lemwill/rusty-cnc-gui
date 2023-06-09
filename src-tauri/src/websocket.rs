use crate::common::{Channels, Data};
use futures::{future::join, SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;

pub async fn handle_websocket(
    channels: Channels,
    mut ws_rx: UnboundedReceiver<Data>,
) -> Result<(), Box<dyn std::error::Error + Send>> {
    // Bind a TCP listener to the address
    let listener = TcpListener::bind("localhost:8081").await.unwrap();

    loop {
        let ws_stream = loop {
            // Accept an incoming TCP connection
            let (stream, _) = listener.accept().await.unwrap();
            // Convert TCP stream to WebSocket stream
            match accept_async(stream).await {
                Ok(ws_stream) => {
                    println!("New WebSocket connection");
                    break ws_stream;
                }
                Err(e) => {
                    println!("Error during the websocket handshake occurred: {}", e);
                    continue;
                }
            }
        };

        let (mut write, mut read) = ws_stream.split();

        let ws_to_usb = channels.ws_to_usb.clone();

        tokio::spawn(async move {
            let (mut ws_tx, mut ws_rx) = futures::channel::mpsc::unbounded::<Data>();

            let read_handle = tokio::spawn(async move {
                // Listen for new messages
                while let Some(result) = read.next().await {
                    match result {
                        Ok(message) => {
                            match message {
                                Message::Binary(data) => {
                                    // Send the data to the USB handler
                                    match ws_to_usb.send(Data { data }) {
                                        Ok(_) => {}
                                        Err(e) => {
                                            println!("Error sending data to USB: {}", e);
                                            break;
                                        }
                                    }
                                }
                                Message::Ping(ping) => {
                                    // Send pong message to writer task via channel
                                    ws_tx
                                        .unbounded_send(Data { data: ping })
                                        .expect("Failed to send Pong");
                                }
                                _ => {}
                            }
                        }
                        Err(e) => {
                            println!("Error reading from WebSocket: {}", e);
                            break;
                        }
                    }
                }
            });

            let write_handle = tokio::spawn(async move {
                println!("WebSocket connection established");

                // Handle websocket data
                while let Some(data) = ws_rx.next().await {
                    // Print the data
                    println!("{:?}", data);
                    match write
                        .send(Message::Binary(data.data))
                        .await
                        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send>)
                    {
                        Ok(_) => {}
                        Err(e) => {
                            println!("Error occurred: {}", e);
                            break; // Break out of this loop and accept a new connection
                        }
                    };
                }
            });

            // Wait for both tasks to complete
            let _ = join(read_handle, write_handle).await;
        });
    }
}
