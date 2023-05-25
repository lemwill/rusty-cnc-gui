use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
#[derive(Debug)]
pub struct Data {
    pub data: Vec<u8>,
}

// The channels used for communication
pub struct Channels {
    pub usb_to_ws: UnboundedSender<Data>,
    pub ws_to_usb: UnboundedSender<Data>,
}
