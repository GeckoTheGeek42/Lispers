#![feature(phase)]
extern crate lispers;

extern crate test;

// #[phase(plugin)]
// extern crate regex_macros;
// extern crate regex;

use lispers::parser::{parse_line, ParserToken};
use lispers::types::LispToken;
use lispers::eval::ExecutionEnvironment;

use test::Bencher;

fn main() {
	test_all();
}

fn test_all() {
	let test_str = "$concat ($sqrt (+ (- 72.0 (* 19 &foo)) (* &a &b &c &d &e &f)))) 'wassup bro' 'idk man' 'this lisp thing is wierd'";
	let parser_tokens = test_parsing(test_str, true);
	let lisp_tokens = test_typing(&parser_tokens, true);
	let result = test_exec(&lisp_tokens, true);
}

fn test_parsing(test_str: &str, print: bool) -> ParserToken {
	let parser_tokens = parse_line(test_str);
		
	if print {
		parser_tokens.pretty_print(&String::new());
		println!("\n====\n====\n");
	}

	return parser_tokens;
}

fn test_typing(parser_tokens: &ParserToken, print: bool) -> LispToken {
	let lisp_tokens = parser_tokens.to_lisp_token();
	
	if print {
		println!("");
		lisp_tokens.pretty_print(&String::new());
		println!("\n====\n====\n");
	}

	return lisp_tokens;
}

fn test_exec(lisp_tokens: &LispToken, print: bool) {
	let env = ExecutionEnvironment::new();
	let result = env.exec(vec![lisp_tokens.clone()]);
} 

#[bench]
fn bench_exec(t: &mut Bencher) {
	let test_str = "$concat ($sqrt (+ (- 72.0 (* 19 &foo)) (* &a &b &c &d &e &f)))) 'wassup bro' 'idk man' 'this lisp thing is wierd'";
	let typed = test_typing( &test_parsing(test_str, false), false );
	t.iter(|| { test_exec(&typed, false) });
}

#[bench]
fn bench_parse(t: &mut Bencher) {
	let test_str = "$concat ($sqrt (+ (- 72.0 (* 19 &foo)) (* &a &b &c &d &e &f)))) 'wassup bro' 'idk man' 'this lisp thing is wierd'";
	t.iter(|| { test_parsing(test_str, false) })
}

#[bench]
fn bench_types(t: &mut Bencher) {
	let test_str = "$concat ($sqrt (+ (- 72.0 (* 19 &foo)) (* &a &b &c &d &e &f)))) 'wassup bro' 'idk man' 'this lisp thing is wierd'";
	let parsed = test_parsing(test_str, false);
	t.iter(|| { test_typing( &parsed, false ) })
}