use hyper::{Body, Request, Response, Server, Uri};
use hyper::service::Service;
use std::collections::HashMap;
use super::config::Config;

pub struct ProxyServer {
    server: Server,
    config: &'_ Config,
}

struct ProxyService {

}

impl ProxyServer {
    pub fn new(config: &Config) {
        let addr = ([127, 0, 0, 1], config.general.port).into();
        let mut s = Server::bind(&addr)
            .serve(|| { ProxyService::new() });   
    }
}

impl ProxyService {
    pub fn new() -> Self {
        ProxyService {}
    }
}

impl Service for ProxyService {
    fn call(&mut self, req: Request<Body>) -> Response<Body> {
        let path_parts: Vec<&str> = req.uri().path().split("/").collect();
        
        Response::new("Hello world")
    }
}