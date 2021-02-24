use std::env;
use std::net::TcpListener;

mod funcs;

fn main() {
	let args :Vec<String> = env::args().collect();

	if args.len() < 3 {
		println!("{}", funcs::help());
		return;
	}

	let listener = TcpListener::bind( &args[1] ).unwrap();

	for x in listener.incoming() {
		let x = x.unwrap();

		funcs::con_handler(x, args[2].clone());
	}
}