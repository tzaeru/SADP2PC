extern crate rustty;

use std::io::prelude::*;
use std::net::TcpStream;
use std::borrow::Cow;

fn main() {
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    rustbox.print(1, 1, rustbox::RB_BOLD, Color::White, Color::Black, "Hello, world!");
    rustbox.print(1, 3, rustbox::RB_BOLD, Color::White, Color::Black,
                  "Press 'q' to quit.");
    rustbox.present();

    let mut stream = TcpStream::connect("127.0.0.1:9123").unwrap();
    //let _ = stream.write(&[1]); // ignore the Result
    //let _ = stream.write(&[1]); // ignore the Result

	let mut buf = vec![0; 512];

    loop {
		let read = stream.read(&mut buf).unwrap();

		let as_string: String = String::from_utf8(buf.clone()).unwrap();
		println!("{}", as_string);

		let _ = stream.write(&[1]);

		match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char('q') => { break; }
                    _ => { }
                }
            },
            Err(e) => panic!("{}", e.description()),
            _ => { }
        }
	}
}
