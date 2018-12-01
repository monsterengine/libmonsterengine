use hyper::{Body, Chunk, header::{ContentLength, ContentType}, server::{Http, NewService, Request, Response, const_service, service_fn}};
use monster_engine_config::MonsterEngineConfig;
use plamo::{PlamoApp, plamo_app_execute, PlamoHttpMethod, PlamoScheme, PlamoByteArray, plamo_request_new, plamo_byte_array_new, plamo_byte_array_get_body_size};
use std::ffi::CString;
use std::ptr::NonNull;
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
    let monster_engine_server_wrapper = MonsterEngineServerWrapper(NonNull::new(monster_engine_server).unwrap());

    let http = Http::<Chunk>::new();
    unsafe {
    let service = const_service(service_fn(move |req: Request| {
        req.body()
            .concat2()
            .map(|_| {
                // let body = "test";
                let path = CString::new("/").unwrap();
                let version = CString::new("1.1").unwrap();
                let plamo_byte_array = plamo_byte_array_new(std::ptr::null(), 0);
                let plamo_request = plamo_request_new(PlamoHttpMethod::Get, PlamoScheme::Http, path.as_ptr(), version.as_ptr(), plamo_byte_array);
                // let monster_engine_server_ptr = monster_engine_server_wrapper.0.as_ptr();
                // let plamo_response = plamo_app_execute((*monster_engine_server_ptr).app, plamo_request);
                // Response::<Body>::new()
                //     .with_header(ContentLength(plamo_byte_array_get_body_size((*plamo_response).body) as u64))
                //     .with_header(ContentType::plaintext())
                //     .with_body(body)
                let body = "test";
                Response::<Body>::new()
                    .with_header(ContentLength(body.len() as u64))
                    .with_header(ContentType::plaintext())
                    .with_body(body)
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
}
