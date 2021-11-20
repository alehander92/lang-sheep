use crate::ast::{Node, NodeFields};
use crate::types::Type;

use crate::errors::LangSheepError;

// ( print "faith" ) -> lparen name expr rparen -> call
//   Call(Name("print"), vec![String("faith")])

#[derive(Debug, Clone, PartialEq)]
enum TokenKind {
	LParen,
	Name,
	String,
	RParen,
	None
}

#[derive(Debug, Clone)]
struct Token {
	kind: TokenKind,
	content: String
}

fn tokenize(source: String) -> Result<Vec<Token>, LangSheepError> {
	let mut tokens = vec![];
	let mut in_token = false;

	// placeholder
	let mut current = Token {
		kind: TokenKind::None,
		content: "".to_string() 
	};

	for c in source.chars() {
		println!("{:?}", c);
		match c {
			'(' => {
				if in_token {
					tokens.push(current.clone());
					in_token = false;
				}
				tokens.push(Token { 
					kind: TokenKind::LParen,
					content: "(".to_string() 
				});
			},
			')' => {
				if in_token {
					tokens.push(current.clone());
					in_token = false;
				}
				tokens.push(Token { 
					kind: TokenKind::RParen,
					content: ")".to_string() 
				});
			},
			' ' => {
				if in_token {
					tokens.push(current.clone());
					in_token = false;
				}
			}
			'a' ..= 'z' => {
				if !in_token {
					current = Token {
						kind: TokenKind::Name,
						content: format!("{}", c)
					};
					in_token = true;
				} else {
					current.content.push(c);
				}
			},
			'"'=> {
				if in_token {
					if current.kind == TokenKind::String {
						tokens.push(current.clone());
						in_token = false;
					} else {
						println!("{:?}", current);
						// TODO better parse error
						return Err(LangSheepError::ParseError)
					}
				} else {
					current = Token {
						kind: TokenKind::String,
						content: "".to_string()
					};
					in_token = true;
				}
			}
			_ => {

			}
		}
	// if source.len() == 0 {
		// Ok(vec![])
	}
	Ok(tokens)
}

fn parse_expr(tokens: Vec<Token>) -> Result<(Node, usize), LangSheepError> {
	if tokens.len() > 0 {
		if tokens[0].kind == TokenKind::Name ||
		   tokens[0].kind == TokenKind::String {
		   	let node_fields = match tokens[0].kind {
		   		TokenKind::Name => NodeFields::Name(tokens[0].content.clone()),
		   		TokenKind::String => NodeFields::String(tokens[0].content.clone()),
		   		_ => unreachable!("token must be name or string")
		   	};
			let node = Node { 
				node: node_fields,
				lang_type: Type::Unknown
			};
			return Ok((node, 0))
		} 
	}
	Err(LangSheepError::ParseError)
}

fn parse_in_parens(tokens: Vec<Token>) -> Result<(Node, usize), LangSheepError> {
	assert!(tokens[0].kind == TokenKind::LParen);
	if tokens.len() < 3 {
		return Err(LangSheepError::ParseError);
	}
	if tokens[1].kind != TokenKind::Name {
		return Err(LangSheepError::ParseError);
	}

	let mut args = vec![];

	// for i in 2 ..= tokens.len - 1 {
	let mut i = 2;
	while i <= tokens.len() - 1 {
		if tokens[i].kind != TokenKind::RParen {
			let (node, index) = parse_expr(tokens[i ..].to_vec())?;
			i += index + 1;
			args.push(node);
		} else {
			let name_node = Node { 
				node: NodeFields::Name(tokens[1].content.clone()),
				lang_type: Type::Unknown
			};
			let call_node = Node {
				node: NodeFields::Call(Box::new(name_node), args),
				lang_type: Type::Unknown
			};
			return Ok((call_node, i));
		}
	}
	// no rparen
	Err(LangSheepError::ParseError)
}

fn parse_single(source: String) -> Result<Node, LangSheepError> {
	let tokens = tokenize(source)?;
	println!("{:?}", tokens);
	if tokens.len() == 0 {
		return Ok(Node { node: NodeFields::None, lang_type: Type::Unknown })
	} else {
		match tokens[0].kind {
			TokenKind::LParen => {
				let (node, index) = parse_in_parens(tokens.clone())?;
				if index == tokens.len() - 1 {
					return Ok(node)
				} else {
					return Err(LangSheepError::ParseError)
				}
			},
			_ => {
				return Err(LangSheepError::ParseError)
			}
		}
	}
	// Err(LangSheepError::ParseError)
}

pub fn parse(source: String) -> Result<Node, LangSheepError> {	
	// (print "faith")
	if source.len() == 0 {
		Ok(Node { node: NodeFields::Program(vec![]), lang_type: Type::Unknown })
	} else {
		let node = parse_single(source)?;
		Ok(Node { node: NodeFields::Program(vec![node]), lang_type: Type::Unknown })
	}
}