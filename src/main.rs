use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0;1024];
    stream.read(&mut buffer).expect("failed to read message");
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("request received: {}", &request);
    let response = "Hello client".as_bytes();
    stream.write(response).expect("failed write response");
}
fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000")?;
    println!("server listening on port: 3000");

    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            },
            Err(e) => eprintln!("failed to handle client, error: {}", e)
        }
    }
    Ok(())
}
