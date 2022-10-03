use candev::Socket;
use embedded_can::{blocking::Can, Frame, StandardId};

fn main() {
    let mut socket = Socket::new("vcan0").unwrap();

    let frame = Frame::new(StandardId::new(1).unwrap(), &[0xDE, 0xAD, 0xBE, 0xFF]).unwrap();
    socket.transmit(&frame).unwrap();
}
