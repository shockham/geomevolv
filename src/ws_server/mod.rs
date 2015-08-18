use std::thread;
use websocket::{Server, Message, Sender, Receiver};
use websocket::header::WebSocketProtocol;
use time::precise_time_s;

/// start the web socket server
pub fn start_ws_server() {
    // Start listening for WebSocket connections
    let ws_server = Server::bind("127.0.0.1:2794").unwrap();

    for connection in ws_server {
        // Spawn a new thread for each connection.
        thread::spawn(move || {
            let request = connection.unwrap().read_request().unwrap(); // Get the request
            let headers = request.headers.clone(); // Keep the headers so we can check them

            request.validate().unwrap(); // Validate the request

            let mut response = request.accept(); // Form a response

            if let Some(&WebSocketProtocol(ref protocols)) = headers.get() {
                if protocols.contains(&("rust-websocket".to_string())) {
                    // We have a protocol we want to use
                    response.headers.set(WebSocketProtocol(vec!["rust-websocket".to_string()]));
                }
            }

            let mut client = response.send().unwrap(); // Send the response

            let ip = client.get_mut_sender()
                .get_mut()
                .peer_addr()
                .unwrap();

            println!("Connection from {}", ip);

            //let message = Message::Text("Hello".to_string());
            //client.send_message(message).unwrap();

            let (mut sender, mut receiver) = client.split();

            for message in receiver.incoming_messages() {
                let message = message.unwrap();

                match message {
                    Message::Close(_) => {
                        let message = Message::Close(None);
                        sender.send_message(message).unwrap();
                        println!("Client {} disconnected", ip);
                        return;
                    }
                    Message::Ping(data) => {
                        let message = Message::Pong(data);
                        sender.send_message(message).unwrap();
                    }
                    _ => {
                        let points = Message::Text(generate_vector_string());
                        sender.send_message(points).unwrap()
                    },
                }
            }
        });
    }
}

/// function for generating the vectors
fn generate_vector_string() -> String {
    let mut s = String::new();
    s.push_str("points = [");

    let mut i:f64 = 0.0;
    let time = precise_time_s();

    for _ in 0..100 {
        let x = ((i * 0.2).sin() * 15.0 + 50.0 * time.cos()) * (time / 2.0).sin();
        let z = ((i - 5.0) * 2.0 * time.tan()) * (time / 10.0).cos(); 

        s = s + &format!( "new THREE.Vector3({},{},{}),", x, 0, z * 5.0);

        i += 1.0;
    }

    s.push_str("];");

    s
}
