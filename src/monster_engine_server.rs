use crate::monster_engine_config::MonsterEngineConfig;
use hyper::{Body, Request, Response, Server, Version};
use hyper::header::{HeaderMap, HeaderValue};
use hyper::http::uri::Scheme;
use hyper::service::service_fn;
use hyper::rt::{self, Future, Stream};
use plamo::*;
use std::ffi::CString;
use std::ptr::NonNull;
use std::slice;
use std::sync::Arc;

struct MonsterEngineServer {
    app: NonNull<PlamoApp>,
    config: NonNull<MonsterEngineConfig>,
}

unsafe impl Send for MonsterEngineServer {}
unsafe impl Sync for MonsterEngineServer {}

#[no_mangle]
pub extern fn monster_engine_server_start(app: *mut PlamoApp, config: *mut MonsterEngineConfig) {
    let monster_engine_server = Arc::new(MonsterEngineServer { app: NonNull::new(app).unwrap(), config: NonNull::new(config).unwrap() });
    let addr = unsafe { (monster_engine_server.config.as_ref()).bind.to_str().unwrap().parse().unwrap() };

    let server = Server::bind(&addr)
        .serve(move || {
            let monster_engine_server = Arc::clone(&monster_engine_server);
            service_fn(move |request: Request<Body>| {
                let uri = request.uri().clone();
                let scheme = uri.scheme_part().map_or(PlamoScheme::Http, |scheme| if scheme == &Scheme::HTTP { PlamoScheme::Http } else { PlamoScheme::Https });
                let path = CString::new(request.uri().path()).unwrap();
                let headers = request.headers().clone();
                let version = match request.version() {
                    Version::HTTP_2 => PlamoHttpVersion::Http20,
                    Version::HTTP_11 => PlamoHttpVersion::Http11,
                    Version::HTTP_10 => PlamoHttpVersion::Http10,
                    Version::HTTP_09 => PlamoHttpVersion::Http09,
                };

                let monster_engine_server = Arc::clone(&monster_engine_server);

                let plamo_http_method = CString::new(request.method().as_str()).unwrap();
                let fut = request.into_body().concat2().and_then(move |body|{
                    let plamo_byte_array = unsafe { plamo_byte_array_new(body.as_ptr(), body.len()) };
                    let plamo_http_query = query(uri.query());
                    let plamo_http_header = header(&headers);
                    let plamo_request = unsafe { plamo_request_new(plamo_http_method.as_ptr(), scheme, path.as_ptr(), version, plamo_http_query, plamo_http_header, plamo_byte_array) };
                    let plamo_response = unsafe { plamo_app_execute(monster_engine_server.app.as_ref(), plamo_request) };
                    Ok(
                        Response::builder()
                            .status(unsafe { (*plamo_response).status_code as u16 })
                            .body(Body::from(unsafe { slice::from_raw_parts(plamo_byte_array_get_body((*plamo_response).body), plamo_byte_array_get_body_size((*plamo_response).body)) }))
                            .unwrap()
                    )
                });

                Box::new(fut)
            })
        })
        .map_err(|e| eprintln!("server error: {}", e));

    rt::run(server);
}

fn query(query: Option<&str>) -> *mut PlamoHttpQuery {
    let plamo_http_query = unsafe { plamo_http_query_new() };
    query.map(|query| {
        query.split("&").for_each(|query| {
            let key_value: Vec<&str> = query.split("=").collect();
            let key = CString::new(key_value[0]).unwrap();
            if key_value.len() == 2 {
                let value = CString::new(key_value[1]).unwrap();
                unsafe { plamo_http_query_add(plamo_http_query, key.as_ptr(), value.as_ptr()); }
            } else {
                unsafe { plamo_http_query_add(plamo_http_query, key.as_ptr(), std::ptr::null()); }
            }
        });
    });
    plamo_http_query
}

fn header(header: &HeaderMap<HeaderValue>) -> *mut PlamoHttpHeader {
    let plamo_http_header = unsafe { plamo_http_header_new() };
    header.iter().for_each(|(key, value)| {
        let key = CString::new(key.as_str()).unwrap();
        let value = CString::new(value.to_str().unwrap()).unwrap();
        unsafe { plamo_http_header_add(plamo_http_header, key.as_ptr(), value.as_ptr()); }
    });
    plamo_http_header
}
