use crate::hal::can;
use crate::{CanError, CanErrorDecodingFailure, ConstructionError};

/// if set, indicate 29 bit extended format
pub const EFF_FLAG: u32 = 0x80000000;

/// remote transmission request flag
pub const RTR_FLAG: u32 = 0x40000000;

/// error flag
pub const ERR_FLAG: u32 = 0x20000000;

/// valid bits in standard frame id
pub const SFF_MASK: u32 = 0x000007ff;

/// valid bits in extended frame id
pub const EFF_MASK: u32 = 0x1fffffff;

/// valid bits in error frame
pub const ERR_MASK: u32 = 0x1fffffff;

/// Frame
///
/// Uses the same memory layout as the underlying kernel struct for performance
/// reasons.
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Frame {
    /// 32 bit CAN_ID + EFF/RTR/ERR flags
    id: u32,

    /// data length. Bytes beyond are not valid
    dlc: u8,

    /// padding
    pad: u8,

    /// reserved
    res0: u8,

    /// reserved
    res1: u8,

    /// buffer for data
    data: [u8; 8],
}

impl Frame {
    pub(crate) fn new_empty() -> Self{
        Frame {
            id: 0,
            dlc: 0,
            pad: 0,
            res0: 0,
            res1: 0,
            data: [0; 8],
        }
    }

    pub fn new(id: u32, data: &[u8], rtr: bool, err: bool) -> Result<Frame, ConstructionError> {
        let mut id = id;

        if data.len() > 8 {
            return Err(ConstructionError::TooMuchData);
        }

        if id > EFF_MASK {
            return Err(ConstructionError::IDTooLarge);
        }

        // set EFF_FLAG on large message
        if id > SFF_MASK {
            id |= EFF_FLAG;
        }

        if rtr {
            id |= RTR_FLAG;
        }

        if err {
            id |= ERR_FLAG;
        }

        let mut full_data = [0; 8];

        // not cool =/
        for (n, c) in data.iter().enumerate() {
            full_data[n] = *c;
        }

        Ok(Frame {
            id: id,
            dlc: data.len() as u8,
            pad: 0,
            res0: 0,
            res1: 0,
            data: full_data,
        })
    }

    pub fn data(&self) -> &[u8] {
        &self.data[..(self.dlc as usize)]
    }

    /// Return the error message
    pub fn err(&self) -> u32 {
        self.id & ERR_MASK
    }

    /// Check if frame is an error message
    pub fn is_error(&self) -> bool {
        self.id & ERR_FLAG != 0
    }

    /// Read error from message and transform it into a `CanError`.
    ///
    /// SocketCAN errors are indicated using the error bit and coded inside
    /// id and data payload. Call `error()` converts these into usable
    /// `CanError` instances.
    ///
    /// If the frame is malformed, this may fail with a
    /// `CanErrorDecodingFailure`.
    pub fn error(&self) -> Result<CanError, CanErrorDecodingFailure> {
        CanError::from_frame(self)
    }
}

impl can::Frame for Frame {
    /// Creates a new frame with a standard identifier.
    fn new_standard(id: u32, data: &[u8]) -> Self {
        //FIXME: this doesn't look right
        Self::new(id, data, false, false).unwrap()
    }

    /// Creates a new frame with an extended identifier.
    fn new_extended(id: u32, data: &[u8]) -> Self {
        //FIXME: this doesn't look right
        Self::new(id, data, false, false).unwrap()
    }

    fn id(&self) -> u32 {
        if self.is_extended() {
            self.id & EFF_MASK
        } else {
            self.id & SFF_MASK
        }
    }

    fn is_extended(&self) -> bool {
        self.id & EFF_FLAG != 0
    }

    fn dlc(&self) -> usize {
        self.dlc as usize
    }

    fn data(&self) -> &[u8] {
        &self.data[..(self.dlc as usize)]
    }

    fn with_rtr(&mut self, dlc: usize) -> &mut Self {
        self.id |= RTR_FLAG;
        self.dlc = dlc as u8;
        self
    }

    fn is_remote_frame(&self) -> bool {
        self.id & RTR_FLAG != 0
    }
}
