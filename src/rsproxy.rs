use async_trait::async_trait;
use crate::config::RouteFile;
use pingora::prelude::*;

pub struct RSProxy {
    pub route_file: RouteFile
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
