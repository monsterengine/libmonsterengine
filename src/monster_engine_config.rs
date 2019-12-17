use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_uint};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct MonsterEngineConfig {
    pub bind: CString,
    pub workers: c_uint,
}

#[no_mangle]
pub extern fn monster_engine_config_new(bind: *const c_char, workers: c_uint) -> *mut MonsterEngineConfig {
    unsafe {
        Box::into_raw(
            Box::new(
                MonsterEngineConfig {
                    bind: CStr::from_ptr(bind).to_owned(),
                    workers: workers,
                }
            )
        )
    }
}

#[no_mangle]
pub extern fn monster_engine_config_destroy(monster_engine_config: *mut MonsterEngineConfig) {
    unsafe {
       drop(Box::from_raw(monster_engine_config));
    }
}
