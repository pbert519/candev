use crate::{CanError, ConstructionError, DecodingError};
use libc::{CAN_EFF_FLAG, CAN_EFF_MASK, CAN_ERR_FLAG, CAN_ERR_MASK, CAN_RTR_FLAG, CAN_SFF_MASK};

/// Frame
///
/// Uses the same memory layout as the underlying kernel struct for performance
/// reasons.
#[derive(Debug, Copy, Clone, Default)]
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
    pub fn new(id: u32, data: &[u8], rtr: bool, err: bool) -> Result<Frame, ConstructionError> {
        let mut id = id;

        if data.len() > 8 {
            return Err(ConstructionError::TooMuchData);
        }

        if id > CAN_EFF_MASK {
            return Err(ConstructionError::IDTooLarge);
        }

        // set EFF_FLAG on large message
        if id > CAN_SFF_MASK {
            id |= CAN_EFF_FLAG;
        }

        if rtr {
            id |= CAN_RTR_FLAG;
        }

        if err {
            id |= CAN_ERR_FLAG;
        }

        let mut full_data = [0; 8];

        // not cool =/
        for (n, c) in data.iter().enumerate() {
            full_data[n] = *c;
        }

        Ok(Frame {
            id,
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
        self.id & CAN_ERR_MASK
    }

    /// Check if frame is an error message
    pub fn is_error(&self) -> bool {
        self.id & CAN_ERR_FLAG != 0
    }

    /// Read error from message and transform it into a `CanError`.
    ///
    /// SocketCAN errors are indicated using the error bit and coded inside
    /// id and data payload. Call `error()` converts these into usable
    /// `CanError` instances.
    ///
    /// If the frame is malformed, this may fail with a
    /// `DecodingError`.
    pub fn error(&self) -> Result<CanError, DecodingError> {
        CanError::from_frame(self)
    }
}

impl embedded_can::Frame for Frame {
    /// Creates a new frame with an extended identifier.
    fn new(id: impl Into<embedded_can::Id>, data: &[u8]) -> Option<Self> {
        match id.into() {
            embedded_can::Id::Extended(value) => match Self::new(value.as_raw(), data, false, false) {
                Ok(frame) => Some(frame),
                _ => None,
            },
            embedded_can::Id::Standard(value) => {
                match Self::new(value.as_raw() as u32, data, false, false) {
                    Ok(frame) => Some(frame),
                    _ => None,
                }
            }
        }
    }

    fn new_remote(id: impl Into<embedded_can::Id>, dlc: usize) -> Option<Self> {
        let data = vec![0; dlc];
        match id.into() {
            embedded_can::Id::Extended(value) => match Self::new(value.as_raw(), &data, true, false) {
                Ok(frame) => Some(frame),
                _ => None,
            },
            embedded_can::Id::Standard(value) => {
                match Self::new(value.as_raw() as u32, &data, true, false) {
                    Ok(frame) => Some(frame),
                    _ => None,
                }
            }
        }
    }

    fn id(&self) -> embedded_can::Id {
        if self.is_extended() {
            embedded_can::Id::Extended(embedded_can::ExtendedId::new(self.id & CAN_EFF_MASK).unwrap())
        } else {
            embedded_can::Id::Standard(embedded_can::StandardId::new((self.id & CAN_SFF_MASK) as u16).unwrap())
        }
    }

    fn is_extended(&self) -> bool {
        self.id & CAN_EFF_FLAG != 0
    }

    fn dlc(&self) -> usize {
        self.dlc as usize
    }

    fn data(&self) -> &[u8] {
        &self.data[..(self.dlc as usize)]
    }

    fn is_remote_frame(&self) -> bool {
        self.id & CAN_RTR_FLAG != 0
    }
}
