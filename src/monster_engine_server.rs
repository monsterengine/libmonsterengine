use crate::monster_engine_config::MonsterEngineConfig;
use hyper::{Body, Request, Response, Server, Version};
use hyper::header::{HeaderMap, HeaderValue};
use hyper::http::method::Method;
use hyper::http::uri::Scheme;
use hyper::service::{make_service_fn, service_fn};
use plamo::*;
use std::ffi::CString;
use std::net::TcpListener;
use std::ptr::NonNull;
use std::slice;
use std::sync::Arc;
use tokio::runtime::Runtime;

pub struct MonsterEngineServer {
    app: NonNull<PlamoApp>,
    tcp_listener: TcpListener,
}

pub struct PlamoAppWrapper(NonNull<PlamoApp>);

unsafe impl Send for PlamoAppWrapper {}
unsafe impl Sync for PlamoAppWrapper {}

#[no_mangle]
pub extern fn monster_engine_server_new(app: *mut PlamoApp, config: *mut MonsterEngineConfig) -> *mut MonsterEngineServer {
    let tcp_listener = unsafe { TcpListener::bind((*config).bind.to_string_lossy().into_owned()).unwrap() };
    Box::into_raw(
        Box::new(
            MonsterEngineServer { app: NonNull::new(app).unwrap(), tcp_listener }
        )
    )
}

#[no_mangle]
pub extern fn monster_engine_server_destroy(monster_engine_server: *mut MonsterEngineServer) {
    unsafe {
       drop(Box::from_raw(monster_engine_server));
    }
}

#[no_mangle]
pub extern fn monster_engine_server_start(monster_engine_server: *const MonsterEngineServer) {
    let mut rt = Runtime::new().unwrap();
    let plamo_app_wrapper = unsafe { Arc::new(PlamoAppWrapper((*monster_engine_server).app.clone())) };
    let tcp_listener = unsafe { (*monster_engine_server).tcp_listener.try_clone().unwrap() };

    let service = make_service_fn(move |_| {
        let plamo_app_wrapper = Arc::clone(&plamo_app_wrapper);
        async {
            Ok::<_, hyper::Error>(service_fn(move |request: Request<Body>| {
                let plamo_app_wrapper = Arc::clone(&plamo_app_wrapper);
                async move {
                    let uri = request.uri().clone();
                    let scheme = uri.scheme().map_or(PlamoScheme::PlamoSchemeHttp, |scheme| if scheme == &Scheme::HTTP { PlamoScheme::PlamoSchemeHttp } else { PlamoScheme::PlamoSchemeHttps });
                    let path = CString::new(request.uri().path()).unwrap();
                    let headers = request.headers().clone();
                    let version = match request.version() {
                        Version::HTTP_2 => PlamoHttpVersion::PlamoHttpVersionHttp20,
                        Version::HTTP_11 => PlamoHttpVersion::PlamoHttpVersionHttp11,
                        Version::HTTP_10 => PlamoHttpVersion::PlamoHttpVersionHttp10,
                        Version::HTTP_09 => PlamoHttpVersion::PlamoHttpVersionHttp09,
                        _ => unimplemented!(),
                    };
                    let plamo_http_method = match request.method() {
                        &Method::GET => PlamoHttpMethod { defined_http_method: PLAMO_HTTP_METHOD_GET },
                        &Method::POST => PlamoHttpMethod { defined_http_method: PLAMO_HTTP_METHOD_POST },
                        &Method::PUT => PlamoHttpMethod { defined_http_method: PLAMO_HTTP_METHOD_PUT },
                        &Method::DELETE => PlamoHttpMethod { defined_http_method: PLAMO_HTTP_METHOD_DELETE },
                        &Method::HEAD => PlamoHttpMethod { defined_http_method: PLAMO_HTTP_METHOD_HEAD },
                        &Method::CONNECT => PlamoHttpMethod { defined_http_method: PLAMO_HTTP_METHOD_CONNECT },
                        &Method::OPTIONS => PlamoHttpMethod { defined_http_method: PLAMO_HTTP_METHOD_OPTIONS },
                        &Method::TRACE => PlamoHttpMethod { defined_http_method: PLAMO_HTTP_METHOD_TRACE },
                        &Method::PATCH => PlamoHttpMethod { defined_http_method: PLAMO_HTTP_METHOD_PATCH },
                        other => PlamoHttpMethod { undefined_http_method: CString::new(other.as_str()).unwrap().into_raw() },
                    };
                    let body = hyper::body::to_bytes(request.into_body()).await.unwrap();
                    let plamo_byte_array = unsafe { plamo_byte_array_new(body.as_ptr(), body.len()) };
                    let plamo_http_query = query(uri.query());
                    let plamo_http_header = header(&headers);
                    let plamo_request = unsafe { plamo_request_new(scheme, version, plamo_http_method, path.as_ptr(), plamo_http_query, plamo_http_header, plamo_byte_array) };
                    let plamo_response = unsafe { plamo_app_execute(plamo_app_wrapper.0.as_ref(), plamo_request) };
                    Ok::<_, hyper::Error>(
                        Response::builder()
                            .status(unsafe { (*plamo_response).status_code as u16 })
                            .body(Body::from(unsafe { slice::from_raw_parts(plamo_byte_array_get_body((*plamo_response).body), plamo_byte_array_get_body_size((*plamo_response).body)) }))
                            .unwrap()
                    )
                }
            }))
        }
    });

    rt.block_on(async {
        Server::from_tcp(tcp_listener).unwrap().serve(service).await.unwrap();
    });
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
