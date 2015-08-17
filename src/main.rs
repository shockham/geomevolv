extern crate websocket;
#[macro_use] extern crate nickel;
extern crate time;

mod http_server;
use http_server::start_http_server;

mod ws_server;
use ws_server::start_ws_server;

fn main() {
    start_http_server();
    start_ws_server(); 
}
