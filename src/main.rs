use std::io::prelude::*;
use std::net::TcpStream;

const ECHO_SERVER_ADDRESS: &str = "localhost:1234";

fn main() {
    // connection
    println!("connecting to tcp://{}", ECHO_SERVER_ADDRESS);

    if let Ok(mut tcp_stream_connect) = TcpStream::connect(ECHO_SERVER_ADDRESS) {
        println!("connected: {:?}", tcp_stream_connect);
        println!(
            "connected to echo server {}:{}",
            tcp_stream_connect.local_addr().unwrap().ip(),
            tcp_stream_connect.local_addr().unwrap().port(),
        );

        let message = "Hello World!";
        let _ = tcp_stream_connect.write(message.as_bytes());
        let _ = tcp_stream_connect.flush();

        // read the result
        let mut buffer = [0; 1024];
        let len = tcp_stream_connect.read(&mut buffer).unwrap();
    } else {
        println!("failed to connect");
        // return;
    }
}
