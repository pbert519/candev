use candev::Socket;
use embedded_hal::can::{nb::Can, Frame};

fn main() {
    let mut socket = Socket::new("vcan0").unwrap();

    let frame = socket.receive().unwrap();
    print!("{:?}#", frame.id());

    let data = frame.data();
    for i in 0..data.len() {
        print!("{:X}", data[i]);
    }
    println!();
}
