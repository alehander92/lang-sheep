use crate::ast::{Node, NodeFields};
use crate::env::Env;
use crate::types::Type;
use crate::LangSheepError;

fn check_type(type_arg: Type, expected_type: Type) -> Result<(), LangSheepError> {
	if type_arg == expected_type {
		Ok(())
	} else {
		Err(LangSheepError::TypeError)
	}
}
fn check_call(typed_function: Node, typed_args: Vec<Node>, env: Env<Type>) -> Result<Type, LangSheepError> {
	println!("check_call {:?} {:?}", typed_function, typed_args);
	match typed_function.lang_type {
		Type::Function(arg_types, return_type) => {
			if arg_types.len() != typed_args.len() {
				return Err(LangSheepError::TypeError);
			}
			for (typed_arg, expected_arg_type) in typed_args.iter().zip(arg_types.iter()) {
				check_type(typed_arg.lang_type.clone(), expected_arg_type.clone())?;
			}
			return Ok(*return_type);
		},
		_ => {
			return Err(LangSheepError::TypeError)
		}
	}
}

fn check_node(node: Node, env: Env<Type>) -> Result<Node, LangSheepError> {
	println!("check_node {:?}", node);
	match node.node {
		NodeFields::Call(function, args) => {
			let typed_function = check_node(*function, env.clone())?;
			let mut typed_args = vec![];
			for arg in args {
				typed_args.push(check_node(arg, env.clone())?);
			}
			let return_type = check_call(typed_function.clone(), typed_args.clone(), env)?;
			Ok(Node {
				node: NodeFields::Call(Box::new(typed_function.clone()), typed_args),
				lang_type: return_type
			})
			// Err(LangSheepError::TypeError)
		},
		NodeFields::Name(name) => {
			match env.get(&name) {
				Some(name_type) => {
					return Ok(Node {
						node: NodeFields::Name(name),
						lang_type: (name_type.clone())
					})
				},
				None => {
					return Err(LangSheepError::TypeError)
				}
			}
			// Err(LangSheepError::TypeError)
		},
		NodeFields::String(ref _text) => {
			Ok(Node {
				node: node.node.clone(),
				lang_type: Type::Simple("String".to_string())
			})
		}
		_ => {
			Err(LangSheepError::TypeError)
		}
	}
}

pub fn typecheck(node: Node, env: Env<Type>) -> Result<Node, LangSheepError> {
	match node.node {
		NodeFields::Program(items) => {
			// let mut typed_items = items;
			let mut typed_items = vec![];
			for item in items {
				typed_items.push(check_node(item, env.clone())?);
			}
			Ok(Node { node: NodeFields::Program(typed_items), lang_type: Type::Empty })
		},
		_ => {
			// not supported non-program
			Err(LangSheepError::TypeError)
		}
	}
}
