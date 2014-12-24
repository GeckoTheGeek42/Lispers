use std::collections::HashMap;
use types::{LispToken, LispFunc};

macro_rules! lisp_func(
	($ee:ident, $args:ident => $fun:expr) => (LispFunc::new(box |$ee, args| -> LispToken {
		match args {
			&LispToken::List(ref $args) => $fun,
			_ => panic!("Invalid Argument Error")
		}
	}))
);

macro_rules! lisp_fold_func(
	( $ee:ident, $args:ident => $start:expr :: $acc:ident, $elem:ident => $fun:expr : $temp:ident => $wrap:expr ) => 
		( lisp_func!($ee, $args => { 
			let $temp = $args.iter().fold( $start, |$acc, $elem| { $fun } );
			$wrap
		}) );
	( $ee:ident, $args:ident => $acc:ident, $elem:ident => $fun:expr : $temp:ident => $wrap:expr ) =>
		( lisp_func!($ee, $args => { 
			let $temp = args.iter().skip(1).fold( $args[0], |$acc, $elem| { $fun } );
			$wrap 
	} ) );
);

macro_rules! lisp_map_func(
	($ee:ident, $args:ident => $arg:ident => $map:expr) => 
		( lisp_func!( $ee, $args => LispToken::List(args.iter().map(|$arg| { $map }.collect()) ) ) )
);

macro_rules! lisp_filter_func(
	($ee:ident, $args:ident => $arg:ident => $filter:expr) => 
		( lisp_func!( $ee, $args => LispToken::List(args.iter().map(|$arg| { $filter }.collect()) ) ) )
);

macro_rules! check_arg(
	($elem:ident, $arg:pat => $then:expr) => (match $elem {
		$arg => $then,
		_ => panic!("Invalid Argument Error")
	})
);

macro_rules! math_fold_funct(
	($a:ident, $n:ident => $fun:expr) => { lisp_func!( ee, arg_list => match arg_list[0] {
				LispToken::Integer(i) => LispToken::Integer(arg_list.iter().skip(1).fold(i, |$a, elem| {
					match elem {
						&LispToken::Integer($n) => $fun,
						&LispToken::FloatingPoint(f) => {
							let $n = f as i32;
							$fun
						},
						_ => panic!("Invalid Argument Error")
					}
				})),
				LispToken::FloatingPoint(f) => LispToken::FloatingPoint(arg_list.iter().skip(1).fold(f, |$a, elem| {
					match elem {
						&LispToken::Integer(i) => {
							let $n = i as f64;
							$fun
						},
						&LispToken::FloatingPoint($n) => $fun,
						_ => panic!("Invalid Argument Error")
					}
				})),
				_ => panic!("Invalid Argument Error")
			})
	}
);

pub fn init_functs(functs: &mut HashMap<&str, LispFunc>) {
	functs.insert("+", math_fold_funct!(a, n => a + n));
	functs.insert("*", math_fold_funct!(a, n => a * n));
	functs.insert("-", math_fold_funct!(a, n => a - n));
	functs.insert("/", math_fold_funct!(a, n => a / n));	
	functs.insert("concat", 
		lisp_fold_func!(ee, args => 
			"".to_string() :: acc, elem => 
				{ acc + elem.as_str().as_slice() } 
					: temp => LispToken::String(temp) )
	);
}