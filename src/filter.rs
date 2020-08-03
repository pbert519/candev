use crate::hal::can;

// EFF/SFF is set in the MSB
const CAN_EFF_FLAG: u32 = 0x80000000;

// standard frame format (SFF)
const CAN_SFF_MASK: u32 = 0x000007ff;

// extended frame format (EFF)
const CAN_EFF_MASK: u32 = 0x1fffffff;

// remote transmission request
const CAN_RTR_FLAG: u32 = 0x40000000;

/// Filter
///
/// A filter matches, when
///     <received_can_id> & mask == can_id & mask
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Filter {
    id: u32,
    mask: u32,
}

impl Filter {
    /// Construct a new CAN filter.
    pub fn new(id: u32, mask: u32) -> Self {
        Filter { id: id, mask: mask }
    }
}

impl can::Filter for Filter {
    fn accept_all() -> Self {
        Self { id: 0, mask: 0 }
    }

    fn new_standard(id: u32) -> Self {
        Self::new(id, CAN_EFF_FLAG | CAN_RTR_FLAG | CAN_SFF_MASK)
    }

    fn new_extended(id: u32) -> Self {
        Self::new(
            id | CAN_EFF_FLAG,
            CAN_EFF_FLAG | CAN_RTR_FLAG | CAN_EFF_MASK,
        )
    }

    fn with_mask(&mut self, mask: u32) -> &mut Self {
        self.mask = mask;
        self
    }

    fn allow_remote(&mut self) -> &mut Self {
        self.id |= CAN_RTR_FLAG;
        self
    }

    fn remote_only(&mut self) -> &mut Self {
        //TODO: not sure how to do this
        todo!()
    }
}
