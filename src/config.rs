use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Route {
    pub prefix: String,
    pub host: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RouteFile {
    pub routes: Vec<Route>
}

