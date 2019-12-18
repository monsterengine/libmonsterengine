use monsterengine::monster_engine_config::*;
use monsterengine::monster_engine_server::*;
use plamo::*;
use std::ffi::CString;
use std::os::raw::c_void;

unsafe extern "C" fn callback(_config: *const c_void, _request: *const PlamoRequest, response: *mut PlamoResponse) {
    let body = "test123".as_bytes();
    let plamo_byte_array = plamo_byte_array_new(body.as_ptr(), body.len());
    (*response).body = plamo_byte_array;
}

fn main() {
    let bind = CString::new("0.0.0.0:8888").unwrap();
    let config = monster_engine_config_new();
    monster_engine_config_set_bind(config, bind.as_ptr());
    let app = unsafe { plamo_app_new() };
    let middleware = unsafe { plamo_middleware_new(std::ptr::null(), Some(callback)) };
    unsafe { plamo_app_add_middleware(app, middleware); }
    let monster_engine_server = monster_engine_server_new(app, config);
    monster_engine_server_start(monster_engine_server);
}
