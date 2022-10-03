use candev::Socket;
use embedded_can::{nb::Can, Frame, StandardId};
use std::thread;

#[derive(Debug)]
struct Driver<T> {
    pub id: u32,
    pub t: T,
}

impl<T, F, E> Driver<T>
where
    F: Frame,
    E: core::fmt::Debug,
    T: embedded_can::nb::Can<Frame = F, Error = E>,
{
    pub fn echo(&mut self) {
        loop {
            if let Ok(frame) = self.t.receive() {
                self.t.transmit(&frame).unwrap();
            }
        }
    }
}

fn main() {
    let socket1 = Socket::new("vcan0").unwrap();
    let mut driver1 = Driver { id: 1, t: socket1 };
    let child1 = thread::spawn(move || {
        driver1.echo();
    });

    let socket2 = Socket::new("vcan0").unwrap();
    let mut driver2 = Driver { id: 2, t: socket2 };
    let child2 = thread::spawn(move || {
        let frame = Frame::new(
            StandardId::new(driver2.id as u16).unwrap(),
            &[0xDE, 0xAD, 0xBE, 0xFF],
        )
        .unwrap();
        driver2.t.transmit(&frame).unwrap();
        driver2.echo();
    });

    child1.join().unwrap();
    child2.join().unwrap();
}
