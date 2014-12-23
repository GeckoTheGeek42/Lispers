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
}

impl LispToken {
	pub fn as_str(&self) -> String {
		match self {
			&LispToken::Integer(i) => i.to_string(),
			&LispToken::FloatingPoint(f) => f.to_string(),
			&LispToken::Boolean(b) => b.to_string(),
			&LispToken::String(ref s) => s.clone(),
			&LispToken::Variable(ref s) => s.clone(),
			&LispToken::Function(ref s) => s.clone(),
			&LispToken::Symbol(ref s) => s.clone(),
			&LispToken::List(ref l) => l.to_string(),
		}
	}

	pub fn from_parser_token(token: &mut ParserToken) -> LispToken {
		match token {
			&ParserToken::Symbol(ref s) => from_str(s.as_slice()).unwrap(),
			&ParserToken::List(ref mut l) => LispToken::List( l.iter_mut().map( |e| LispToken::from_parser_token(e) ).collect::<Vec<LispToken>>() ),
		}
	}
}

impl FromStr for LispToken {
	fn from_str(s: &str) -> Option<LispToken> {
		from_str(s).map(|e| LispToken::Integer(e))
			.or(from_str(s).map(|e| LispToken::FloatingPoint(e)))
			.or(from_str(s).map(|e| LispToken::Boolean(e)))
			.or(regex!(r"^'(.*)'$").captures(s).map(|c| LispToken::String(c.at(1).to_string())))
			.or(regex!(r"^\$(.*)$").captures(s).map(|c| LispToken::Variable(c.at(1).to_string())))
			.or(regex!(r"^:(.*)$").captures(s).map(|c| LispToken::Function(c.at(1).to_string())))
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
		}
	}
}

pub struct LispFunc<'a> {
	pub funct: Box<Fn(&ExecutionEnvironment, &LispToken) -> LispToken + 'a>
}