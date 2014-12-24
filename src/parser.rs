#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

use types::LispToken;
use {debug, debugln};

#[deriving(Show)]
pub enum ParserToken {
	Symbol(String),
	List(Vec<ParserToken>),
}

impl ParserToken {
	pub fn pretty_print(&self, indent: &String) {
		match self {
			&ParserToken::Symbol(ref s) => println!("{}|- {}", indent, s),
			&ParserToken::List(ref v) => {
				println!("{}.<---", indent);
				for t in v.iter() { t.pretty_print( &format!("..{}", indent) ) }
				println!("{}.--->", indent);
			}
		}
	}

	pub fn to_lisp_token(&self) -> LispToken {
		LispToken::from_parser_token(self)
	}
}

#[deriving(Show)]
pub struct ParserBuffer {
	paren_count: u8,
	in_quote: bool,
	token: String,
	tokens: Vec<ParserToken>,
}

impl ParserBuffer {
	fn new() -> ParserBuffer {
		ParserBuffer {
			in_quote: false,
			paren_count: 0,
			tokens: Vec::new(),
			token: String::new(),
		}
	}

	fn push_open_paren(&mut self) {
		debug("( "); 
		if self.paren_count != 0 {
			self.token.push('(');
		}
		self.paren_count = self.paren_count + 1;
	}

	fn push_close_paren(&mut self) {
		debug(") "); 
		if self.paren_count == 1 {
			debugln("\npushin list"); 
			self.paren_count = 0;
			self.tokens.push( parse_line( self.token.as_slice() ) );
			self.token = String::new();
		} else {
			self.paren_count = self.paren_count - 1;
			self.token.push(')');
		}
	}

	fn push_quote(&mut self) {
		self.token.push('\'');
		if self.in_quote {
			debug("' "); 
			self.in_quote = false;
			debugln(format!("\npushin symbol:'{}'", self.token.clone()).as_slice()); 
			self.tokens.push( ParserToken::Symbol(self.token.clone()) );
			self.token = String::new();	
		} else {
			debug("` "); 
			self.in_quote = true
		}
	}

	fn push_char(&mut self, c: char) {
		debug(format!("{} ", c).as_slice()); 
		self.token.push(c);
	}

	fn push_symbol(&mut self) {
		if self.token == "" {
			return;
		}
		if self.in_quote {
			debug("_ "); 
			self.token.push(' ');
			return;
		}
		if self.paren_count > 0 {
			debug("_ "); 
			self.token.push(' ');
			return;
		}
		debugln(format!("\npushin symbol:'{}'", self.token.clone()).as_slice()); 
		self.tokens.push(ParserToken::Symbol(self.token.clone()));
		self.token = String::new();
	}
}

pub fn parse_lines(code_str: &str) -> Vec<ParserToken> {
	code_str.split_str(";").map(|s| {
		parse_line(s)
	}).collect()
}

pub fn parse_line(code_str: &str) -> ParserToken {
	debugln(format!("parsing: {}", code_str).as_slice()); 
	let mut buf = code_str.chars().fold(ParserBuffer::new(), |acc, elem| {
		let mut acc_copy = acc;
		match elem {
			' ' => acc_copy.push_symbol(),
			'(' => acc_copy.push_open_paren(),
			')' => acc_copy.push_close_paren(),
			'\'' => acc_copy.push_quote(),
			'\n' | '\t' => {},
			c => acc_copy.push_char(c),
		};
		acc_copy
	});
	buf.push_symbol();
	return ParserToken::List(buf.tokens);
}