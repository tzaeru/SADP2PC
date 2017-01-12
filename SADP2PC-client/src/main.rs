use std::io;
use std::io::prelude::*;
use std::net::TcpStream;
use std::borrow::Cow;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:9123").unwrap();
    //let _ = stream.write(&[1]); // ignore the Result
    //let _ = stream.write(&[1]); // ignore the Result

	let mut buf = vec![0; 512];

    loop {
		let read = stream.read(&mut buf).unwrap();

		let as_string: String = String::from_utf8(buf.clone()).unwrap();
		println!("{}", as_string);

		let _ = stream.write(&[1]);

		let mut input = String::new();
		match io::stdin().read_line(&mut input) {
		    Ok(n) => {
		        println!("{} bytes read", n);
		        println!("{}", input);

		        if input.chars().nth(0).unwrap() == 'q'
		        {
		        	break;
		        }
		    }
		    Err(error) => println!("error: {}", error),
		}
	}
}
