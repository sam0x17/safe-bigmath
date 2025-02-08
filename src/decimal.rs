#[repr(C)]
pub struct DecimalInternal<const P: usize> {
    data: u64,
}
