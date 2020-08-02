use crate::ConstructionError;

/// Filter
///
/// Contains an internal id and mask. Packets are considered to be matched by
/// a filter if `received_id & mask == filter_id & mask` holds true.
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Filter {
    _id: u32,
    _mask: u32,
}

impl Filter {
    /// Construct a new CAN filter.
    pub fn new(id: u32, mask: u32) -> Result<Filter, ConstructionError> {
        Ok(Filter {
            _id: id,
            _mask: mask,
        })
    }
}
