#[repr(align(16))]
pub enum HeapInt {
    U120([u8; 15]),
    Ptr { ptr: *mut u8, len: [u8; 7] },
}
