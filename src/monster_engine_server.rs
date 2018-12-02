use hyper::{Body, Chunk, header::{ContentLength, ContentType}, server::{Http, NewService, Request, Response, const_service, service_fn}};
use monster_engine_config::MonsterEngineConfig;
use plamo::{PlamoApp, plamo_app_execute, PlamoHttpMethod, PlamoScheme, PlamoByteArray, plamo_request_new, plamo_byte_array_new, plamo_byte_array_get_body_size, plamo_byte_array_get_body};
use std::ffi::CString;
use std::ptr::NonNull;
use std::slice;
use std::sync::Arc;
use tokio;
use tokio::{net::TcpListener, prelude::*};

#[repr(C)]
pub struct MonsterEngineServer {
    app: *const PlamoApp,
    config: *const MonsterEngineConfig,
}

struct MonsterEngineServerWrapper(NonNull<MonsterEngineServer>);

unsafe impl Send for MonsterEngineServerWrapper {}
unsafe impl Sync for MonsterEngineServerWrapper {}

#[no_mangle]
pub extern fn monster_engine_server_new(app: *const PlamoApp, config: *const MonsterEngineConfig) -> *mut MonsterEngineServer {
    Box::into_raw(Box::new(MonsterEngineServer {
        app: app,
        config: config,
    }))
}

#[no_mangle]
pub extern fn monster_engine_server_start(monster_engine_server: *mut MonsterEngineServer) {
    let addr = unsafe { (*(*monster_engine_server).config).bind.to_str().unwrap().parse().unwrap() };
    let listener = TcpListener::bind(&addr).unwrap();
    let monster_engine_server_wrapper = Arc::new(MonsterEngineServerWrapper(NonNull::new(monster_engine_server).unwrap()));

    let http = Http::<Chunk>::new();
    let service = const_service(service_fn(move |req: Request| {

        let monster_engine_server_wrapper = Arc::clone(&monster_engine_server_wrapper);
        req.body()
            .concat2()
            .map(move |_| {
                let path = CString::new("/").unwrap();
                let version = CString::new("1.1").unwrap();
                let plamo_byte_array = unsafe { plamo_byte_array_new(std::ptr::null(), 0) };
                let plamo_request = unsafe { plamo_request_new(PlamoHttpMethod::Get, PlamoScheme::Http, path.as_ptr(), version.as_ptr(), plamo_byte_array) };
                let monster_engine_server_ptr = unsafe { (*monster_engine_server_wrapper).0.as_ref() };
                let plamo_response = unsafe { plamo_app_execute(monster_engine_server_ptr.app, plamo_request) };
                let body_size = unsafe { plamo_byte_array_get_body_size((*plamo_response).body) };
                Response::<Body>::new()
                    .with_header(ContentLength(body_size as u64))
                    .with_header(ContentType::plaintext())
                    .with_body(unsafe { slice::from_raw_parts(plamo_byte_array_get_body((*plamo_response).body), body_size) })
            })
    }));

    let server = listener.incoming().map_err(|e| eprintln!("accept failed = {:?}", e)).for_each(move |sock| {
        tokio::spawn(
            http.serve_connection(sock, service.new_service().unwrap())
                .map(|_| ())
                .map_err(|_| ())
        )
    });

    tokio::run(server);
}
