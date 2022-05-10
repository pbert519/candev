use embedded_hal as hal;
mod error;
pub use error::{CanError, ConstructionError, DecodingError, SocketError};

// mod filter;
// pub use filter::{Filter, FilterGroup, FilterGroups};

mod frame;
pub use frame::Frame;

mod socket;
pub use socket::Socket;
