use std::os::raw::{c_char, c_int};

pub const ACEGF_METHOD_GENERATE: i32 = 1;
pub const ACEGF_METHOD_WEITOA: i32 = 2;
pub const ACEGF_METHOD_REKEY: i32 = 3;
pub const ACEGF_METHOD_VIEW: i32 = 4;

#[repr(C)]
#[derive(Debug)]
pub struct ACEGF_Call {
    pub method: i32,
    pub input_1: *const c_char,
    pub input_2: *const c_char,
    pub input_3: *const c_char,
}

#[repr(C)]
#[derive(Debug)]
pub struct ACEGF_Result {
    pub code: i32,
    pub data: *const c_char,
    pub data_len: u64,
    pub reserved: [u8; 8],
}
