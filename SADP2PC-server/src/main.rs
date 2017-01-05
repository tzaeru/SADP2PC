use std::io::prelude::*;
use std::net::TcpListener;
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9123").unwrap();
    println!("listening started, ready to accept");
    for stream in listener.incoming() {
        thread::spawn(|| {
            
            
            let mut stream = stream.unwrap();
            stream.write(b"Hello World\r\n").unwrap();

            let mut buf = vec![0; 512];

            loop {
            	let read = stream.read(&mut buf).unwrap();
                stream.write(b"Hello World\r\n").unwrap();
            	if read > 5
                {
            		break;
                }
            }
        });
    }
}
