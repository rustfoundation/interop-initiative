/// A simple struct with mixed-type fields, marked repr(C) so its memory
/// layout is stable and compatible with C.
#[repr(C)]
pub struct Example {
    pub byte: u8,
    pub integer: u32,
    pub flag: bool,
}

/// Returns an Example with known values so the C side can verify
/// it is reading the fields at the correct offsets.
#[unsafe(no_mangle)]
pub extern "C" fn create_example() -> Example {
    Example {
        byte: 42,
        integer: 1000,
        flag: true,
    }
}
