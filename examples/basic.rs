use monsterengine::monster_engine_config::monster_engine_config_new;
use monsterengine::monster_engine_server::monster_engine_server_start;
use plamo::*;
use std::ffi::{CStr, CString};
use std::os::raw::c_void;

unsafe extern "C" fn callback(_config: *const c_void, request: *const PlamoRequest, response: *mut PlamoResponse) {
    let request_body = std::slice::from_raw_parts(plamo_byte_array_get_body((*request).body), plamo_byte_array_get_body_size((*request).body));
    println!("{}", std::str::from_utf8_unchecked(request_body));
    if (*request).method.defined_http_method >= PLAMO_HTTP_METHOD_PATCH {
        println!("{}", (*request).method.defined_http_method);
    } else {
        println!("{:?}", CStr::from_ptr((*request).method.undefined_http_method));
    }
    let body = "test".as_bytes();
    let plamo_byte_array = plamo_byte_array_new(body.as_ptr(), body.len());
    (*response).body = plamo_byte_array;
}

fn main() {
    let bind = CString::new("0.0.0.0:8888").unwrap();
    let config = monster_engine_config_new(bind.as_ptr(), 1);
    let app = unsafe { plamo_app_new() };
    let middleware = unsafe { plamo_middleware_new(std::ptr::null(), Some(callback)) };
    unsafe { plamo_app_add_middleware(app, middleware); }
    monster_engine_server_start(app, config);
}
