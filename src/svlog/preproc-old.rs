// Copyright (c) 2016 Fabian Schuiki

//! A preprocessor for SystemVerilog files that takes the raw stream of
//! tokens generated by a lexer and performs include and macro
//! resolution.
//!
//! It seems that a better approach to the whole preprocessing business would be
//! to have the basic lexer emit all tokens, including comments and whitespaces.
//! Then the Preprocessor would only need to operate on those tokens,
//! identifying includes and macros, and substituting macro uses as appropriate.

use std::path::PathBuf;
use svlog::{lexer, token};
use svlog::lexer::{Lexer, TokenAndSpan};
use errors::{DiagnosticBuilder, DiagResult, DUMMY_HANDLER};
use name::Name;
pub use svlog::token::Token;
use std::collections::HashMap;

pub struct Preprocessor {
	stack: Vec<Box<Lexer>>,
	macros: HashMap<Name,Name>,
}

impl Preprocessor {
	/// Creates a new preprocessor and initial lexer for the given file.
	pub fn new(filename: &str) -> Preprocessor {
		Preprocessor {
			stack: vec![Box::new(lexer::make(filename))],
			macros: HashMap::new(),
		}
	}

	pub fn next_token<'b>(&mut self) -> DiagResult<'b, TokenAndSpan> {
		'outer: loop {
			let result = self.stack.last_mut().unwrap().next_token();
			return match result {
				Ok(TokenAndSpan{tkn,sp}) => {
					match tkn {
						token::Eof => {
							if self.stack.len() == 1 {
								return Ok(TokenAndSpan { tkn: tkn, sp: sp });
							} else {
								println!("popping lexer");
								self.stack.pop();
								continue;
							}
						},

						// Resolve included files. This is pretty minimal as of
						// now, but is sufficient to handle the simplest include
						// scenarios.
						token::Include(filename) => {
							println!("resolving include {:?}", filename);
							let mut search_paths = Vec::new();

							// Directory the current file is in.
							let mut dir = PathBuf::from(self.stack.last().unwrap().get_path());
							dir.pop();
							search_paths.push(dir);

							// Some random other directory.
							let mut dir = PathBuf::from(self.stack.last().unwrap().get_path());
							dir.pop();
							dir.push("includes");
							search_paths.push(dir);

							// Try out all search paths in order and accept the
							// first one that exists.
							for mut path in search_paths {
								path.push(&filename.as_str() as &str);
								if path.exists() {
									println!("pushing lexer for file {}", path.to_str().unwrap());
									self.stack.push(Box::new(lexer::make(path.to_str().unwrap())));
									continue 'outer;
								}
							}

							// TODO: Turn this into a proper error message.
							panic!("unable to resolve include {:?}", filename);
						},

						token::Define(name, body) => {
							println!("storing macro {} definition {}", name, body);
							self.macros.insert(name, body);
							continue;
						},

						token::CompDir(name) => {
							let mc = self.macros.get(&name);
							if let Some(definition) = mc {
								println!("would substitute {} for its definition {}", name, definition);
								continue;
							}

							panic!("compiler directive {} not implemented", name);
						},

						x => Ok(TokenAndSpan { tkn: x, sp: sp }),
					}
				},
				other => other
			}
		}
		// match token.tkn {
		// 	Ok(token::Include(filename)) => {
		// 		println!("resolving include {:?}", filename);
		// 		self.stack.last_mut().unwrap().next_token()
		// 	}
		// 	other => other
		// }
	}
}
