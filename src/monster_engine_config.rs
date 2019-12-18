use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct MonsterEngineConfig {
    pub bind: CString,
}

#[no_mangle]
pub extern fn monster_engine_config_new() -> *mut MonsterEngineConfig {
    Box::into_raw(
        Box::new(
            MonsterEngineConfig {
                bind: CString::new("0.0.0.0:80").unwrap(),
            }
        )
    )
}

#[no_mangle]
pub extern fn monster_engine_config_destroy(monster_engine_config: *mut MonsterEngineConfig) {
    unsafe {
       drop(Box::from_raw(monster_engine_config));
    }
}

#[no_mangle]
pub extern fn monster_engine_config_get_bind(monster_engine_config: *const MonsterEngineConfig) -> *const c_char {
    unsafe {
        (*monster_engine_config).bind.as_ptr()
    }
}

#[no_mangle]
pub extern fn monster_engine_config_set_bind(monster_engine_config: *mut MonsterEngineConfig, bind: *const c_char) {
    unsafe {
        (*monster_engine_config).bind = CStr::from_ptr(bind).to_owned();
    }
}
