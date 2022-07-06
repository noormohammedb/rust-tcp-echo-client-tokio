use tokio::io::AsyncReadExt;
use tokio::{io::AsyncWriteExt, net::TcpStream};

const ECHO_SERVER_ADDRESS: &str = "localhost:1234";

#[tokio::main]
async fn main() {
    // connection
    println!("connecting to tcp://{}", ECHO_SERVER_ADDRESS);

    if let Ok(mut tcp_stream_connect) = TcpStream::connect(ECHO_SERVER_ADDRESS).await {
        println!("connected: {:?}", tcp_stream_connect);
        println!(
            "connected to echo server {}:{}",
            tcp_stream_connect.local_addr().unwrap().ip(),
            tcp_stream_connect.local_addr().unwrap().port(),
        );

        let message = "Hello World!";
        let _ = tcp_stream_connect.write_all(message.as_bytes()).await;
        // let _ = tcp_stream_connect.flush();

        // read the result
        let mut buffer = [0; 1024];
        tcp_stream_connect.read(&mut buffer).await.unwrap();
        let message = String::from_utf8_lossy(&buffer);

        println!("received: {}", message);
    } else {
        println!("failed to connect");
        // return;
    }
}
