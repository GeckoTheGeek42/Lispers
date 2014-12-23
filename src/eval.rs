use std::collections::HashMap;
use types::{LispToken, LispFunc};

pub struct ExecutionEnvironment<'a> {
    variables: HashMap<&'a str, LispToken>,
    functions: HashMap<&'a str, LispFunc<'a>>
}

impl<'a> ExecutionEnvironment<'a> {
	pub fn new() -> ExecutionEnvironment<'a> {
		let functs = HashMap::new();

		ExecutionEnvironment {
			variables: HashMap::new(),
			functions: functs
		}
	}

	pub fn exec(&mut self, script: Vec<LispToken>) -> Vec<LispToken> {
		script.iter()
			.map(|token| self.var_map(token))
			.map(|token| self.eval_expr(&token))
			.collect()
	}

	fn var_map(&self, token: &LispToken) -> LispToken {
		match token {
			&LispToken::Variable(ref v) => self.get_var( v.as_slice() ).unwrap(),
			&LispToken::List(ref l) => LispToken::List( l.iter().map( |t| self.var_map(t) ).collect() ),
			token_else => token_else.clone(),
		}
	}

	fn eval_expr(&self, token: &LispToken) -> LispToken {
		match token.find_executable() {
			LispToken::Executable(f, a) => self.get_fn(f.as_slice()).unwrap().call(self, &LispToken::List(a.iter().map(|t| self.eval_expr(t)).collect())),
			t => t,
		}
	}

	pub fn get_var(&self, k: &str) -> Result<LispToken, &str> {
		self.variables
			.get(k)
			.map(|lt| lt.clone())
			.ok_or("Invalid Variable Name")
	}

	pub fn get_fn(&self, k: &str) -> Result<&LispFunc<'a>, &str> {
		self.functions
			.get(k)
			.ok_or("Invalid Variable Name")
	}

	pub fn add_var(&mut self, k: &'a str, v: LispToken) {
		self.variables.insert(k, v);
	}

	pub fn add_fn(&mut self, k: &'a str, v: LispFunc<'a>) {
		self.functions.insert(k, v);
	}
}