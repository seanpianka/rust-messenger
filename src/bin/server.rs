use std::io;
use std::io::{Read, Write};
use std::net;
use std::thread;


fn handle_client(mut stream: net::TcpStream) -> Result<(), io::Error> {
    let mut buf = [0; 512];

    println!("Incoming connection from: {}", stream.peer_addr()?);

    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 { return Ok(()) }
        stream.write(&buf[..bytes_read])?;
    }
}


fn main() -> io::Result<()> {
    let listener = net::TcpListener::bind("127.0.0.1:4000").expect("Could not bind to addr port"); 

    for stream in listener.incoming() {
        match stream {
            Err(e) => { eprintln!("Failed: {}", e) }
            Ok(stream) => { 
                thread::spawn(|| {
                    handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
        }
    }

    Ok(())
}
