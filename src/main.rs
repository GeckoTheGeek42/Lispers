#![feature(phase)]
extern crate lispers;

// #[phase(plugin)]
// extern crate regex_macros;
// extern crate regex;

use lispers::parser::parse_line;
use lispers::eval::ExecutionEnvironment;

fn main() {
	let test_str = "$concat ($sqrt ($+ ($- 72 ($* 19 &foo)) ($cross_product (&a &b 32) (71 1891 &c) (&d &e &f)) 2983 98234 9823)) 'wassup bro' 'idk man' 'this lisp thing is wierd'";
	let parser_tokens = parse_line(test_str);
		parser_tokens.pretty_print(&String::new());

	println!("\n====\n====\n");

	let lisp_tokens = parser_tokens.to_lisp_token();
		lisp_tokens.pretty_print(&String::new());

	println!("\n====\n====\n");

	let mut env = ExecutionEnvironment::new();
	let result = vec![lisp_tokens];
}