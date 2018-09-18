use hyper::{Body, Chunk, header::{ContentLength, ContentType}, server::{Http, NewService, Request, Response, const_service, service_fn}};
use monster_engine_config::MonsterEngineConfig;
use tokio;
use tokio::{net::TcpListener, prelude::*};

#[repr(C)]
pub struct MonsterEngineServer {
    config: MonsterEngineConfig,
}

#[no_mangle]
pub extern fn monster_engine_server_start(monster_engine_server: *const MonsterEngineServer) {
    let addr = unsafe { (*(*monster_engine_server).config.bind).to_str().unwrap().parse().unwrap() };
    let listener = TcpListener::bind(&addr).unwrap();

    let http = Http::<Chunk>::new();
    let service = const_service(service_fn(|req: Request| {
        req.body()
            .concat2()
            .map(|_| {
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
