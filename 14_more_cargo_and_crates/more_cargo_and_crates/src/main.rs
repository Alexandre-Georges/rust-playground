// This is a workaround for VSCode, the following line should not be necessary
mod lib;
use lib::Color;
use lib::get;

fn main() {
	println!("{}", lib::add_one(1));
	println!("{:?}", Color::Red);
	println!("{:?}", get());
}