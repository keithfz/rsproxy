use async_trait::async_trait;
use config::RouteFile;
use pingora::prelude::*;
use std::fs;
use std::env;
use serde_yaml;

mod config;

pub struct RSProxy {
    route_file: RouteFile
}

#[async_trait]
impl ProxyHttp for RSProxy {

    type CTX = ();
    fn new_ctx(&self) -> () {
        ()
    }

    async fn upstream_peer(&self, _session: &mut Session, _ctx: &mut ()) -> Result<Box<HttpPeer>> {
        let mut upstream: (&str, u16) = ("", 0);
        let mut route_found: bool = false;
        for route in self.route_file.routes.iter() {
            if _session.req_header().uri.path().starts_with(&route.prefix) {
                upstream = (&route.host.as_str(), 443);
                route_found = true;
            }
        }
        if !route_found {
            _session.respond_error(503).await;
            return Err(Error::explain(ConnectProxyFailure, "No upstream"))
        }

        println!("upstream peer is: {upstream:?}");

        // ignore SNI for now
        let peer = Box::new(HttpPeer::new(upstream, true, "".to_string()));
        Ok(peer)
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let port = &args[1];
    let file_path = &args[2];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    // don't unwrap like this in the real world! Errors will result in panic!
    let route_file: RouteFile = serde_yaml::from_str::<RouteFile>(&contents).unwrap();
   
    println!("{:#?}", route_file);

    let mut my_server = Server::new(None).unwrap();
    my_server.bootstrap();

    let proxy: RSProxy = RSProxy { route_file: route_file }; 
    let mut lb = http_proxy_service(&my_server.configuration, proxy);

    let listen_addr = format!("0.0.0.0:{port}");
    println!("Listening on {:#?}", listen_addr);
    lb.add_tcp(&listen_addr);

    my_server.add_service(lb);

    my_server.run_forever();
}

