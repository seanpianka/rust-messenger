use std::io;
use std::io::{BufRead, Write};
use std::net;
use std::str;


fn main() -> io::Result<()> {
    let mut stream = net::TcpStream::connect("127.0.0.1:4000").expect("Could not connect to server");

    loop {
        let mut input: String = String::new();
        let mut buffer: Vec<u8> = Vec::new();
        io::stdin().read_line(&mut input).expect("Failed to read from stdin");

        if input.trim() == "exit" {
            break;
        }

        stream.write(input.as_bytes()).expect("Failed to write to server");

        let mut reader = io::BufReader::new(&stream);

        reader.read_until(b'\n', &mut buffer).expect("Could not read into buffer");
        print!("{}", str::from_utf8(&buffer).expect("Could not write buffer as string"));
    }

    Ok(())
}
