#![feature(unboxed_closures)]
#![feature(phase)]
#![feature(macro_rules)]
extern crate regex;

pub mod types;
pub mod parser;
pub mod eval;
pub mod init;

fn debug(s: &str) {
	// print!("{}", s);
}
fn debugln(s: &str) {
	// println!("{}", s);
}