use candev::Socket;
use embedded_hal::can::{Frame, Transmitter};

fn main() {
    let mut socket = Socket::new("vcan0").unwrap();

    let frame = <Socket as Transmitter>::Frame::new_standard(1, &[1]).unwrap();
    socket.transmit(&frame).unwrap();
}
