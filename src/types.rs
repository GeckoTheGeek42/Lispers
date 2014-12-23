use std::str::FromStr;

use parser::ParserToken;
use eval::ExecutionEnvironment;

#[deriving(Show, PartialEq)]
pub enum LispToken {
	Integer(i32),
	FloatingPoint(f64),
	Boolean(bool),
	String(String),//'.*'
	Variable(String),//$.*
	Function(String),//:.*
	Symbol(String),//.* ??
	List(Vec<LispToken>),
	Executable(String, Vec<LispToken>),
}

impl LispToken {
	pub fn pretty_print(&self, indent: &String) {
		match self {
			&LispToken::List(ref l) => { 
				println!("{}<---", indent);
				for t in l.iter() { 
					t.pretty_print(&format!("..{}", indent)) 
				} 
				println!("{}--->", indent);
			},
			&LispToken::Executable(ref f, ref a) => {
				println!("{}Function({}) -->", indent, f);
				println!("{}.<---", indent);
				for t in a.iter() { 
					t.pretty_print(&format!("..{}", indent)) 
				} 
				println!("{}.--->", indent);
			},
			t => println!("{}|- {}", indent, t)
		}
	}

	pub fn as_str(&self) -> String {
		match self {
			&LispToken::Integer(i) => i.to_string(),
			&LispToken::FloatingPoint(f) => f.to_string(),
			&LispToken::Boolean(b) => b.to_string(),
			&LispToken::String(ref s) => format!("'{}'", s),
			&LispToken::Variable(ref s) => format!("&{}", s),
			&LispToken::Function(ref s) => format!("${}", s),
			&LispToken::Symbol(ref s) => s.clone(),
			&LispToken::List(ref l) => l.to_string(),
			&LispToken::Executable(ref f, ref a) => format!("({} {})", f, a),
		}
	}

	pub fn from_parser_token(token: &ParserToken) -> LispToken {
		match token {
			&ParserToken::Symbol(ref s) => from_str(s.as_slice()).unwrap(),
			&ParserToken::List(ref l) => LispToken::List( 
				l.iter()
					.map( |e| LispToken::from_parser_token(e) )
					.collect::<Vec<LispToken>>() 
			),
		}.find_executable()
	}

	pub fn find_executable(&self) -> LispToken {
		match self {
			&LispToken::List(ref l) => match l[0] {
				LispToken::Function(ref f) => LispToken::Executable(
					f.clone(), 
					l.iter()
						.skip(1)
						.map(|ref t| t.find_executable())
						.collect()
				),
				_ => LispToken::List(
					l.clone()
						.iter()
						.map(|ref t| t.find_executable())
						.collect()
				)
			},
			t => t.clone(),
		}
	}
}

impl FromStr for LispToken {
	fn from_str(s: &str) -> Option<LispToken> {
		let str_re = regex!(r"^'(.*)'$");
		let var_re = regex!(r"^&(.*)$");
		let func_re = regex!(r"^\$(.*)$");

		from_str(s).map(|e| LispToken::Integer(e))
			.or(from_str(s).map(|e| LispToken::FloatingPoint(e)))
			.or(from_str(s).map(|e| LispToken::Boolean(e)))
			.or(str_re.captures(s).map(|c| LispToken::String(c.at(1).to_string())))
			.or(var_re.captures(s).map(|c| LispToken::Variable(c.at(1).to_string())))
			.or(func_re	.captures(s).map(|c| LispToken::Function(c.at(1).to_string())))
			.or(Some(LispToken::Symbol(s.to_string())))
	}
}

impl Clone for LispToken {
	fn clone(&self) -> LispToken {
		match self {
			&LispToken::Integer(i) => LispToken::Integer(i),
			&LispToken::FloatingPoint(f) => LispToken::FloatingPoint(f),
			&LispToken::Boolean(b) => LispToken::Boolean(b),
			&LispToken::String(ref s) => LispToken::String(s.clone()),
			&LispToken::Variable(ref s) => LispToken::Variable(s.clone()),
			&LispToken::Function(ref s) => LispToken::Function(s.clone()),
			&LispToken::Symbol(ref s) => LispToken::Symbol(s.clone()),
			&LispToken::List(ref l) => LispToken::List(l.clone()),
			&LispToken::Executable(ref f, ref a) => LispToken::Executable(f.clone(), a.clone())
		}
	}
}

pub struct LispFunc<'a> {
	pub funct: Box<Fn(&ExecutionEnvironment, &LispToken) -> LispToken + 'a>
}

impl<'a> LispFunc<'a> {
	pub fn new(f: Box<Fn(&ExecutionEnvironment, &LispToken) -> LispToken + 'a>) -> LispFunc<'a> {
		LispFunc {
			funct: f
		}
	}

	pub fn call(&self, ee: &ExecutionEnvironment, args: &LispToken) -> LispToken {
		self.funct.call((ee, args))
	}
}