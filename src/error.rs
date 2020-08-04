use crate::Frame;
use std::convert::TryFrom;

#[derive(Debug, Copy, Clone)]
/// Error that occurs when creating CAN packets
pub enum ConstructionError {
    /// CAN ID was outside the range of valid IDs
    IDTooLarge,
    /// More than 8 Bytes of payload data were passed in
    TooMuchData,
}

/// Helper function to retrieve a specific byte of frame data or returning an
/// `Err(..)` otherwise.
fn get_data(frame: &Frame, idx: u8) -> Result<u8, DecodingError> {
    Ok(*(frame
        .data()
        .get(idx as usize)
        .ok_or_else(|| DecodingError::NotEnoughData(idx)))?)
}

/// Error decoding a CanError from a CanFrame.
#[derive(Copy, Clone, Debug)]
pub enum DecodingError {
    /// The supplied CanFrame did not have the error bit set.
    NotAnError,

    /// The error type is not known and cannot be decoded.
    UnknownErrorType(u32),

    /// The error type indicated a need for additional information as `data`,
    /// but the `data` field was not long enough.
    NotEnoughData(u8),

    /// The error type `ControllerProblem` was indicated and additional
    /// information found, but not recognized.
    InvalidControllerProblem,

    /// The type of the ProtocolViolation was not valid
    InvalidViolationType,

    /// A location was specified for a ProtocolViolation, but the location
    /// was not valid.
    InvalidLocation,

    /// The supplied transciever error was invalid.
    InvalidTransceiverError,
}

#[derive(Copy, Clone, Debug)]
pub enum CanError {
    /// TX timeout (by netdevice driver)
    TransmitTimeout,

    /// Arbitration was lost. Contains the number after which arbitration was
    /// lost or 0 if unspecified
    LostArbitration(u8),

    /// Controller problem, see `ControllerProblem`
    ControllerProblem(ControllerProblem),

    /// Protocol violation at the specified `Location`. See `ProtocolViolation`
    /// for details.
    ProtocolViolation {
        vtype: ViolationType,
        location: Location,
    },

    /// Transceiver Error.
    TransceiverError,

    /// No ACK received for current CAN frame.
    NoAck,

    /// Bus off (due to too many detected errors)
    BusOff,

    /// Bus error (due to too many detected errors)
    BusError,

    /// The bus has been restarted
    Restarted,

    /// Unknown, possibly invalid, error
    Unknown(u32),
}

#[derive(Copy, Clone, Debug)]
pub enum ControllerProblem {
    // unspecified
    Unspecified,

    // RX buffer overflow
    ReceiveBufferOverflow,

    // TX buffer overflow
    TransmitBufferOverflow,

    // reached warning level for RX errors
    ReceiveErrorWarning,

    // reached warning level for TX errors
    TransmitErrorWarning,

    // reached error passive status RX
    ReceiveErrorPassive,

    // reached error passive status TX
    TransmitErrorPassive,

    // recovered to error active state
    Active,
}

impl TryFrom<u8> for ControllerProblem {
    type Error = DecodingError;

    fn try_from(val: u8) -> Result<ControllerProblem, DecodingError> {
        Ok(match val {
            0x00 => ControllerProblem::Unspecified,
            0x01 => ControllerProblem::ReceiveBufferOverflow,
            0x02 => ControllerProblem::TransmitBufferOverflow,
            0x04 => ControllerProblem::ReceiveErrorWarning,
            0x08 => ControllerProblem::TransmitErrorWarning,
            0x10 => ControllerProblem::ReceiveErrorPassive,
            0x20 => ControllerProblem::TransmitErrorPassive,
            0x40 => ControllerProblem::Active,
            _ => return Err(DecodingError::InvalidControllerProblem),
        })
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ViolationType {
    /// Unspecified Violation
    Unspecified,

    /// Single Bit Error
    SingleBitError,

    /// Frame formatting error
    FrameFormatError,

    /// Bit stuffing error
    BitStuffingError,

    /// A dominant bit was sent, but not received
    UnableToSendDominantBit,

    /// A recessive bit was sent, but not received
    UnableToSendRecessiveBit,

    /// Bus overloaded
    BusOverload,

    /// Bus is active (again)
    Active,

    /// Transmission Error
    TransmissionError,
}

impl TryFrom<u8> for ViolationType {
    type Error = DecodingError;

    fn try_from(val: u8) -> Result<ViolationType, DecodingError> {
        Ok(match val {
            0x00 => ViolationType::Unspecified,
            0x01 => ViolationType::SingleBitError,
            0x02 => ViolationType::FrameFormatError,
            0x04 => ViolationType::BitStuffingError,
            0x08 => ViolationType::UnableToSendDominantBit,
            0x10 => ViolationType::UnableToSendRecessiveBit,
            0x20 => ViolationType::BusOverload,
            0x40 => ViolationType::Active,
            0x80 => ViolationType::TransmissionError,
            _ => return Err(DecodingError::InvalidViolationType),
        })
    }
}

/// Location
///
/// Describes where inside a received frame an error occured.
#[derive(Copy, Clone, Debug)]
pub enum Location {
    /// Unspecified
    Unspecified,

    /// Start of frame.
    StartOfFrame,

    /// ID bits 28-21 (SFF: 10-3)
    Id2821,

    /// ID bits 20-18 (SFF: 2-0)
    Id2018,

    /// substitute RTR (SFF: RTR)
    SubstituteRtr,

    /// extension of identifier
    IdentifierExtension,

    /// ID bits 17-13
    Id1713,

    /// ID bits 12-5
    Id1205,

    /// ID bits 4-0
    Id0400,

    /// RTR bit
    Rtr,

    /// Reserved bit 1
    Reserved1,

    /// Reserved bit 0
    Reserved0,

    /// Data length
    DataLengthCode,

    /// Data section
    DataSection,

    /// CRC sequence
    CrcSequence,

    /// CRC delimiter
    CrcDelimiter,

    /// ACK slot
    AckSlot,

    /// ACK delimiter
    AckDelimiter,

    /// End-of-frame
    EndOfFrame,

    /// Intermission (between frames)
    Intermission,
}

impl TryFrom<u8> for Location {
    type Error = DecodingError;

    fn try_from(val: u8) -> Result<Location, DecodingError> {
        Ok(match val {
            0x00 => Location::Unspecified,
            0x03 => Location::StartOfFrame,
            0x02 => Location::Id2821,
            0x06 => Location::Id2018,
            0x04 => Location::SubstituteRtr,
            0x05 => Location::IdentifierExtension,
            0x07 => Location::Id1713,
            0x0F => Location::Id1205,
            0x0E => Location::Id0400,
            0x0C => Location::Rtr,
            0x0D => Location::Reserved1,
            0x09 => Location::Reserved0,
            0x0B => Location::DataLengthCode,
            0x0A => Location::DataSection,
            0x08 => Location::CrcSequence,
            0x18 => Location::CrcDelimiter,
            0x19 => Location::AckSlot,
            0x1B => Location::AckDelimiter,
            0x1A => Location::EndOfFrame,
            0x12 => Location::Intermission,
            _ => return Err(DecodingError::InvalidLocation),
        })
    }
}

pub enum TransceiverError {
    Unspecified,
    CanHighNoWire,
    CanHighShortToBat,
    CanHighShortToVcc,
    CanHighShortToGnd,
    CanLowNoWire,
    CanLowShortToBat,
    CanLowShortToVcc,
    CanLowShortToGnd,
    CanLowShortToCanHigh,
}

impl TryFrom<u8> for TransceiverError {
    type Error = DecodingError;

    fn try_from(val: u8) -> Result<TransceiverError, DecodingError> {
        Ok(match val {
            0x00 => TransceiverError::Unspecified,
            0x04 => TransceiverError::CanHighNoWire,
            0x05 => TransceiverError::CanHighShortToBat,
            0x06 => TransceiverError::CanHighShortToVcc,
            0x07 => TransceiverError::CanHighShortToGnd,
            0x40 => TransceiverError::CanLowNoWire,
            0x50 => TransceiverError::CanLowShortToBat,
            0x60 => TransceiverError::CanLowShortToVcc,
            0x70 => TransceiverError::CanLowShortToGnd,
            0x80 => TransceiverError::CanLowShortToCanHigh,
            _ => return Err(DecodingError::InvalidTransceiverError),
        })
    }
}

impl CanError {
    pub fn from_frame(frame: &Frame) -> Result<CanError, DecodingError> {
        if !frame.is_error() {
            return Err(DecodingError::NotAnError);
        }

        match frame.err() {
            0x00000001 => Ok(CanError::TransmitTimeout),
            0x00000002 => Ok(CanError::LostArbitration(get_data(frame, 0)?)),
            0x00000004 => Ok(CanError::ControllerProblem(ControllerProblem::try_from(
                get_data(frame, 1)?,
            )?)),

            0x00000008 => Ok(CanError::ProtocolViolation {
                vtype: ViolationType::try_from(get_data(frame, 2)?)?,
                location: Location::try_from(get_data(frame, 3)?)?,
            }),

            0x00000010 => Ok(CanError::TransceiverError),
            0x00000020 => Ok(CanError::NoAck),
            0x00000040 => Ok(CanError::BusOff),
            0x00000080 => Ok(CanError::BusError),
            0x00000100 => Ok(CanError::Restarted),
            e => Err(DecodingError::UnknownErrorType(e)),
        }
    }
}

pub trait ControllerSpecificErrorInformation {
    fn get_ctrl_err(&self) -> Option<&[u8]>;
}

impl ControllerSpecificErrorInformation for Frame {
    #[inline]
    fn get_ctrl_err(&self) -> Option<&[u8]> {
        let data = self.data();

        if data.len() != 8 {
            None
        } else {
            Some(&data[5..])
        }
    }
}
