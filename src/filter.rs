use crate::hal::can;
use crate::hal::can::{MaskType, RtrFilterBehavior};
use crate::{
    CAN_EFF_FLAG, CAN_EFF_MASK, CAN_RAW_FILTER, CAN_RAW_FILTER_MAX, CAN_RTR_FLAG, CAN_SFF_MASK,
    SOL_CAN_RAW,
};
use libc::{c_int, c_void, setsockopt, socklen_t};
use std::{mem::size_of, ptr};

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
        self.mask |= CAN_RTR_FLAG;
        self
    }

    fn remote_only(&mut self) -> &mut Self {
        //TODO: not sure how to do this
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct FilterGroup {
    fd: c_int,
    filters: Vec<Filter>,
}

impl FilterGroup {
    pub(crate) fn new(fd: c_int) -> Self {
        FilterGroup {
            fd: fd,
            filters: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.filters.len()
    }

    pub fn add_filter(&mut self, filter: Filter) {
        self.filters.push(filter);
    }

    pub fn clear_filters(&mut self) {
        self.filters.clear();
        self.set_filters(&[]).unwrap();
    }

    /// Sets filters on the socket.
    ///
    /// CAN packages received by SocketCAN are matched against these filters,
    /// only matching packets are returned by the interface.
    ///
    /// See `CanFilter` for details on how filtering works. By default, all
    /// single filter matching all incoming frames is installed.
    fn set_filters(&self, filters: &[Filter]) -> std::io::Result<()> {
        self.set_socket_option_mult(self.fd, SOL_CAN_RAW, CAN_RAW_FILTER, filters)
    }

    fn set_socket_option_mult<T>(
        &self,
        fd: c_int,
        level: c_int,
        name: c_int,
        values: &[T],
    ) -> std::io::Result<()> {
        let rv = if values.len() < 1 {
            // can't pass in a pointer to the first element if a 0-length slice,
            // pass a nullpointer instead
            unsafe { setsockopt(fd, level, name, ptr::null(), 0) }
        } else {
            unsafe {
                let val_ptr = &values[0] as *const T;

                setsockopt(
                    fd,
                    level,
                    name,
                    val_ptr as *const c_void,
                    (size_of::<T>() * values.len()) as socklen_t,
                )
            }
        };

        if rv != 0 {
            return Err(std::io::Error::last_os_error());
        }

        Ok(())
    }
}

impl can::FilterGroup for FilterGroup {
    fn num_filters(&self) -> usize {
        CAN_RAW_FILTER_MAX as usize
    }

    fn extended(&self) -> bool {
        // Filter works for extended (29bit) identifiers
        true
    }

    fn mask(&self) -> Option<MaskType> {
        // There is a configurable mask for each filter
        Some(MaskType::Individual)
    }

    fn rtr(&self) -> RtrFilterBehavior {
        // RTR bit is part of the the filter and the mask
        RtrFilterBehavior::Configurable
    }
}

#[derive(Debug)]
pub struct FilterGroups {}

impl Iterator for FilterGroups {
    type Item = FilterGroup;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        todo!()
    }
}
