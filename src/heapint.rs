use uintx::{u120, u56};

#[repr(align(16))]
pub enum HeapInt {
    U120(u120),
    Ptr { ptr: *mut u8, len: u56 },
}
