use config::RouteFile;
use rsproxy::RSProxy;
use pingora::prelude::*;
use std::fs;
use std::env;
use serde_yaml;

mod config;
mod rsproxy;

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

