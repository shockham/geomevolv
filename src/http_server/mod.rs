use std::thread;
use nickel::{Nickel, HttpRouter, StaticFilesHandler};
use std::collections::HashMap;

/// start the web socket server
pub fn start_http_server() {
    // Start listening for http connections
    thread::spawn(move || {
        let mut server = Nickel::new();
        
        server.utilize(StaticFilesHandler::new("static/"));

        server.get("/", middleware! { |_, res|
            let mut data = HashMap::<&str, &str>::new();
            data.insert("name", "user");
            return res.render("templates/index.tpl", &data)
        });

        server.listen("127.0.0.1:6767");
    });
}
