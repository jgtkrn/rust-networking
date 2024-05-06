use std::io::{Read, Write};
#[allow(unused_imports)]
use std::{error::Error, net::{TcpListener, TcpStream}};

fn handle_client(mut stream: TcpStream){
    loop {
        let mut buffer = [0;1024];
        // let response = b"+PONG\r\n";
        match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(_) => {
                stream.write(buffer.as_slice());
            },
            Err(e) => {
                eprintln!("failed to read stream, {e}");
                break;
            }
        }
    }
}
fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000")?;
    println!("server listening on port: 3000");

    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream),
            Err(e) => eprintln!("failed to handle client, error: {}", e)
        }
    }
    Ok(())
}
