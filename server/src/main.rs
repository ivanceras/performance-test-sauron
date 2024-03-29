#![deny(warnings)]
use std::net::SocketAddr;
use warp::{http::Response, Filter};

// path relative to the working directory when you run the server binary
const PKG_DIR: &str = "client/pkg";
const FAVICON_FILE: &str = "client/favicon.ico";
const INDEX_PAGE: &str = include_str!("../../client/index.html");
const DEFAULT_PORT: u16 = 2337;

#[tokio::main]
async fn main() {
    // The compiled javascript and wasm in the client.
    let pkg_files = warp::path("pkg").and(warp::fs::dir(PKG_DIR));

    let favicon = warp::path("favicon.ico").and(warp::fs::file(FAVICON_FILE));

    let render_page = || Response::builder().body(INDEX_PAGE);

    // Render paths that don't include a name with a default
    let root = warp::path::end().map(move || render_page());

    // These are the example url paths
    // GET
    //   /
    //   /favicon.ico
    //   /pkg/client.js
    //
    let routes = warp::get().and(root.or(favicon).or(pkg_files));

    let port = if let Ok(port) = std::env::var("PORT") {
        if let Ok(port) = port.parse::<u16>() {
            port
        } else {
            DEFAULT_PORT
        }
    } else {
        DEFAULT_PORT
    };

    let socket: SocketAddr = ([0, 0, 0, 0], port).into();
    println!("serve at http://{}:{}", socket.ip(), socket.port());
    warp::serve(routes).run(socket).await;
}
