use std::collections::HashMap;
use types::{LispToken, LispFunc};
use init::{init_functs, init_vars};
use debugln;

pub struct ExecutionEnvironment<'a> {
    variables: HashMap<&'a str, LispToken>,
    functions: HashMap<&'a str, LispFunc<'a>>
}

impl<'a> ExecutionEnvironment<'a> {
	pub fn new() -> ExecutionEnvironment<'a> {
		let mut functs = HashMap::new();
		init_functs(&mut functs);

		let mut vars = HashMap::new();
		init_vars(&mut vars);

		ExecutionEnvironment {
			variables: vars,
			functions: functs
		}
	}

	pub fn exec(&self, script: Vec<LispToken>) -> Vec<LispToken> {
		script.iter()
			.map(|token| self.var_map(token))
			.map(|token| self.eval_expr(&token))
			.collect()
	}

	fn var_map(&self, token: &LispToken) -> LispToken {
		debugln("Var_map: ");
//		token.pretty_print(&String::new());
		match token {
			&LispToken::Variable(ref v) => self.get_var( v.as_slice() ).unwrap(),
			&LispToken::List(ref l) => LispToken::List( l.iter().map( |t| self.var_map(t) ).collect() ),
			&LispToken::Executable(ref f, ref l) => LispToken::Executable(f.clone(), l.iter().map( |t| self.var_map(t) ).collect()),
			token_else => token_else.clone(),
		}
	}

	fn eval_expr(&self, token: &LispToken) -> LispToken {
		let r = match token {
			&LispToken::Executable(ref f, ref a) => 
				self.get_fn( f.as_slice() ).unwrap()
					.call(self, &LispToken::List( a.iter().map(|t| self.eval_expr(t)).collect() )),
			t => t.clone(),
		};

		debugln("Evaluating: "); //DEBUG
//		token.pretty_print(&String::new()); //DEBUG
		debugln(format!(":: {}", r).as_slice());

		return r;
	}

	pub fn get_var(&self, k: &str) -> Result<LispToken, String> {
		self.variables
			.get(k)
			.map(|lt| lt.clone())
			.ok_or(format!("Invalid Variable Error: {}", k))
	}

	pub fn get_fn(&self, k: &str) -> Result<&LispFunc<'a>, String> {
		self.functions
			.get(k)
			.ok_or(format!("Invalid Function Error: {}", k))
	}

	pub fn add_var(&mut self, k: &'a str, v: LispToken) {
		self.variables.insert(k, v);
	}

	pub fn add_fn(&mut self, k: &'a str, v: LispFunc<'a>) {
		self.functions.insert(k, v);
	}
}