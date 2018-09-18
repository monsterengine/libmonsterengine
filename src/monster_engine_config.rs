use std::ffi::CString;
use std::os::raw::c_uint;

#[repr(C)]
pub struct MonsterEngineConfig {
    pub bind: *const CString,
    pub workers: c_uint,
}
