use candev::Socket;
use embedded_hal::can::{Frame, Receiver, Transmitter};
use nb::block;
use std::{thread, time};

#[derive(Debug)]
struct Driver<T> {
    pub id: u32,
    pub t: T,
}

impl<T> Driver<T>
where
    T: Receiver + Transmitter,
    <T as Transmitter>::Error: core::fmt::Debug,
{
    pub fn echo(&mut self) {
        let mut count = 0;
        while count < 3 {
            if let Ok(frame) = self.t.receive() {
                let frame = <T as Transmitter>::Frame::new_standard(self.id, frame.data()).unwrap();
                thread::sleep(time::Duration::from_secs(1));
                block!(self.t.transmit(&frame)).unwrap();
            }

            count += 1;
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
        let frame =
            <Socket as Transmitter>::Frame::new_standard(driver2.id, &[0xDE, 0xAD, 0xBE, 0xFF])
                .unwrap();
        driver2.t.transmit(&frame).unwrap();
        driver2.echo();
    });

    child1.join().unwrap();
    child2.join().unwrap();
}
