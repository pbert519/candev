use libc::c_int;

use embedded_hal as hal;

mod error;
pub use error::{CanError, DecodingError, ConstructionError};

mod filter;
pub use filter::{Filter, FilterGroup, FilterGroups};

mod frame;
pub use frame::Frame;

mod socket;
pub use socket::{Socket, SocketError};

const AF_CAN: c_int = 29;
const PF_CAN: c_int = 29;
const CAN_RAW: c_int = 1;
const SOL_CAN_BASE: c_int = 100;
const SOL_CAN_RAW: c_int = SOL_CAN_BASE + CAN_RAW;
const CAN_RAW_FILTER: c_int = 1;
const CAN_RAW_ERR_FILTER: c_int = 2;
const CAN_RAW_LOOPBACK: c_int = 3;
const CAN_RAW_RECV_OWN_MSGS: c_int = 4;
const CAN_RAW_JOIN_FILTERS: c_int = 6;

/// if set, indicate 29 bit extended format
const EFF_FLAG: u32 = 0x80000000;

/// remote transmission request flag
const RTR_FLAG: u32 = 0x40000000;

/// error flag
const ERR_FLAG: u32 = 0x20000000;

/// valid bits in standard frame id
const SFF_MASK: u32 = 0x000007ff;

/// valid bits in extended frame id
const EFF_MASK: u32 = 0x1fffffff;

/// valid bits in error frame
const ERR_MASK: u32 = 0x1fffffff;

// EFF/SFF is set in the MSB
const CAN_EFF_FLAG: u32 = 0x80000000;

// standard frame format (SFF)
const CAN_SFF_MASK: u32 = 0x000007ff;

// extended frame format (EFF)
const CAN_EFF_MASK: u32 = 0x1fffffff;

// remote transmission request
const CAN_RTR_FLAG: u32 = 0x40000000;

// maximum number of can_filter set via setsockopt()
pub const CAN_RAW_FILTER_MAX: c_int = 512;

