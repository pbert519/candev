use crate::{Frame, SocketError};
use libc::{
    bind, c_int, c_short, c_uint, c_void, close, fcntl, if_nametoindex, read, setsockopt, sockaddr,
    socket, socklen_t, suseconds_t, time_t, timeval, write, AF_CAN, CAN_RAW, CAN_RAW_ERR_FILTER,
    CAN_RAW_JOIN_FILTERS, CAN_RAW_LOOPBACK, CAN_RAW_RECV_OWN_MSGS, F_GETFL, F_SETFL, O_NONBLOCK,
    PF_CAN, SOCK_RAW, SOL_CAN_RAW, /*CAN_RAW_FILTER_MAX*/
    SOL_SOCKET, SO_RCVTIMEO, SO_SNDTIMEO,
};
use std::{
    ffi::CString,
    io,
    // iter::{once, Once},
    mem::size_of,
    time,
};

#[derive(Debug)]
#[repr(C)]
struct CanAddr {
    _af_can: c_short,
    if_index: c_int, // address familiy,
    rx_id: u32,
    tx_id: u32,
}

/// A socket for a CAN device.
///
/// Will be closed upon deallocation. To close manually, use std::drop::Drop.
/// Internally this is just a wrapped file-descriptor.
#[derive(Debug)]
pub struct Socket {
    fd: c_int,
    // filter_group: FilterGroup,
}

impl Socket {
    /// Open a named CAN device.
    pub fn new(ifname: &str) -> Result<Socket, SocketError> {
        let ifname = CString::new(ifname).unwrap();
        let ifindex = unsafe { if_nametoindex(ifname.as_ptr()) };
        if ifindex == 0 {
            return Err(SocketError::IOError(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid interface",
            )));
        }
        Socket::open_if(ifindex)
    }

    /// Open CAN device by interface number.
    ///
    /// Opens a CAN device by kernel interface number.
    pub fn open_if(if_index: c_uint) -> Result<Socket, SocketError> {
        let addr = CanAddr {
            _af_can: AF_CAN as c_short,
            if_index: if_index as c_int,
            rx_id: 0, // ?
            tx_id: 0, // ?
        };

        // open socket
        let sock_fd;
        unsafe {
            sock_fd = socket(PF_CAN, SOCK_RAW, CAN_RAW);
        }

        if sock_fd == -1 {
            return Err(SocketError::from(io::Error::last_os_error()));
        }

        // bind it
        let bind_rv;
        unsafe {
            let sockaddr_ptr = &addr as *const CanAddr;
            bind_rv = bind(
                sock_fd,
                sockaddr_ptr as *const sockaddr,
                size_of::<CanAddr>() as u32,
            );
        }

        // FIXME: on fail, close socket (do not leak socketfds)
        if bind_rv == -1 {
            let e = io::Error::last_os_error();
            unsafe {
                close(sock_fd);
            }
            return Err(SocketError::from(e));
        }

        Ok(Socket {
            fd: sock_fd,
            // filter_group: FilterGroup::new(sock_fd),
        })
    }

    fn close(&mut self) -> io::Result<()> {
        unsafe {
            let rv = close(self.fd);
            if rv != -1 {
                return Err(io::Error::last_os_error());
            }
        }
        Ok(())
    }

    /// Change socket to non-blocking mode
    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        // retrieve current flags
        let oldfl = unsafe { fcntl(self.fd, F_GETFL) };

        if oldfl == -1 {
            return Err(io::Error::last_os_error());
        }

        let newfl = if nonblocking {
            oldfl | O_NONBLOCK
        } else {
            oldfl & !O_NONBLOCK
        };

        let rv = unsafe { fcntl(self.fd, F_SETFL, newfl) };

        if rv != 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }

    /// Sets the read timeout on the socket
    ///
    /// For convenience, the result value can be checked using
    /// `ShouldRetry::should_retry` when a timeout is set.
    pub fn set_read_timeout(&self, duration: time::Duration) -> io::Result<()> {
        self.set_socket_option(self.fd, SOL_SOCKET, SO_RCVTIMEO, &c_timeval_new(duration))
    }

    /// Sets the write timeout on the socket
    pub fn set_write_timeout(&self, duration: time::Duration) -> io::Result<()> {
        self.set_socket_option(self.fd, SOL_SOCKET, SO_SNDTIMEO, &c_timeval_new(duration))
    }

    /// Sets the error mask on the socket.
    ///
    /// By default (`ERR_MASK_NONE`) no error conditions are reported as
    /// special error frames by the socket. Enabling error conditions by
    /// setting `ERR_MASK_ALL` or another non-empty error mask causes the
    /// socket to receive notification about the specified conditions.
    pub fn set_error_mask(&self, mask: u32) -> io::Result<()> {
        self.set_socket_option(self.fd, SOL_CAN_RAW, CAN_RAW_ERR_FILTER, &mask)
    }

    /// Enable or disable loopback.
    ///
    /// By default, loopback is enabled, causing other applications that open
    /// the same CAN bus to see frames emitted by different applications on
    /// the same system.
    pub fn set_loopback(&self, enabled: bool) -> io::Result<()> {
        let loopback: c_int = if enabled { 1 } else { 0 };
        self.set_socket_option(self.fd, SOL_CAN_RAW, CAN_RAW_LOOPBACK, &loopback)
    }

    /// Enable or disable receiving of own frames.
    ///
    /// When loopback is enabled, this settings controls if CAN frames sent
    /// are received back immediately by sender. Default is off.
    pub fn set_recv_own_msgs(&self, enabled: bool) -> io::Result<()> {
        let recv_own_msgs: c_int = if enabled { 1 } else { 0 };
        self.set_socket_option(self.fd, SOL_CAN_RAW, CAN_RAW_RECV_OWN_MSGS, &recv_own_msgs)
    }

    /// Enable or disable join filters.
    ///
    /// By default a frame is accepted if it matches any of the filters set
    /// with `set_filters`. If join filters is enabled, a frame has to match
    /// _all_ filters to be accepted.
    pub fn set_join_filters(&self, enabled: bool) -> io::Result<()> {
        let join_filters: c_int = if enabled { 1 } else { 0 };
        self.set_socket_option(self.fd, SOL_CAN_RAW, CAN_RAW_JOIN_FILTERS, &join_filters)
    }

    fn set_socket_option<T>(
        &self,
        fd: c_int,
        level: c_int,
        name: c_int,
        val: &T,
    ) -> io::Result<()> {
        let rv = unsafe {
            let val_ptr: *const T = val as *const T;

            setsockopt(
                fd,
                level,
                name,
                val_ptr as *const c_void,
                size_of::<T>() as socklen_t,
            )
        };
        if rv != 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
}

impl embedded_can::blocking::Can for Socket {
    type Frame = Frame;
    type Error = SocketError;

    fn transmit(&mut self, frame: &Frame) -> Result<(), Self::Error> {
        // not a mutable reference needed (see std::net::UdpSocket) for
        // a comparison
        // debug!("Sending: {:?}", frame);

        let write_rv = unsafe {
            let frame_ptr = frame as *const Frame;
            write(self.fd, frame_ptr as *const c_void, size_of::<Frame>())
        };

        if write_rv as usize != size_of::<Frame>() {
            return Err(SocketError::from(io::Error::last_os_error()));
        }

        Ok(())
    }

    fn receive(&mut self) -> Result<Self::Frame, Self::Error> {
        let mut frame = Frame::default();
        let nbytes = unsafe {
            let frame_ptr = &mut frame as *mut Frame;
            read(self.fd, frame_ptr as *mut c_void, size_of::<Frame>())
        };

        if nbytes as usize != size_of::<Frame>() {
            let e = io::Error::last_os_error();
            return Err(SocketError::IOError(e));
        }

        Ok(frame)
    }
}

impl Drop for Socket {
    fn drop(&mut self) {
        self.close().ok(); // ignore result
    }
}

fn c_timeval_new(t: time::Duration) -> timeval {
    timeval {
        tv_sec: t.as_secs() as time_t,
        tv_usec: t.subsec_micros() as suseconds_t,
    }
}

#[cfg(test)]
mod tests {
    use crate::Socket;

    #[test]
    fn test_nonexistant_device() {
        assert!(Socket::new("invalid").is_err());
    }

    #[cfg(feature = "vcan0")]
    mod vcan {
        use crate::Socket;
        use embedded_can::{blocking::Can, Frame};
        use embedded_can::{Id, StandardId};
        use libc::CAN_ERR_MASK;

        const VCAN0: &str = "vcan0";
        /// error mask that will instruct the socket to report all errors
        pub const ERR_MASK_ALL: u32 = CAN_ERR_MASK;
        /// an error mask that will instruct the socket to drop all errors
        pub const ERR_MASK_NONE: u32 = 0x00000000;

        #[test]
        fn vcan0_set_error_mask() {
            let socket = Socket::new(VCAN0).unwrap();
            socket.set_error_mask(ERR_MASK_ALL).unwrap();
            socket.set_error_mask(ERR_MASK_NONE).unwrap();
        }

        #[test]
        fn vcan0_enable_own_loopback() {
            let id = Id::Standard(StandardId::new(0x123).unwrap());
            let data: &[u8] = &[0xDE, 0xAD, 0xBE, 0xFF];
            let mut socket = Socket::new(VCAN0).unwrap();
            socket.set_loopback(true).unwrap();
            socket.set_recv_own_msgs(true).unwrap();

            let frame = Frame::new(id, data).unwrap();

            socket.transmit(&frame).unwrap();

            let frame = socket.receive().unwrap();
            assert_eq!(frame.id(), id);
            assert_eq!(frame.data(), data);
        }

        #[test]
        fn vcan0_test_nonblocking() {
            let mut socket = Socket::new(VCAN0).unwrap();
            socket.set_nonblocking(true).unwrap();

            // no timeout set: should return immediately
            match socket.receive() {
                Ok(_) => assert!(false),
                Err(e) => match e {
                    crate::SocketError::IOError(err)
                        if err.kind() == std::io::ErrorKind::WouldBlock => {}
                    _ => assert!(false),
                },
            }
        }
    }
}
