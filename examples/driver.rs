use candev::Socket;
use embedded_hal::can::{Frame, Receiver, Transmitter};
use std::{thread, time};

struct Driver<T> {
    pub id: u32,
    pub t: T,
}

impl<T> Driver<T>
where
    T: Receiver + Transmitter,
{
    pub fn echo(&mut self) {
        let mut count = 0;
        while count < 3 {
            /* This doesn't work:
            if let Ok(frame) = self.t.receive() {
                self.t.transmit(&frame);
            }
            */
            if let Ok(frame) = self.t.receive() {
                let frame = <T as Transmitter>::Frame::new_standard(self.id, frame.data()).unwrap();
                thread::sleep(time::Duration::from_secs(1));
                //FIXME: Adding .unwrap() gives an error???
                match self.t.transmit(&frame) {
                    Ok(f) => match f {
                        None => {}
                        Some(_) => panic!("Transmit returned a queued frame"),
                    },
                    Err(_) => panic!("Failed to transmit frame"),
                }
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
