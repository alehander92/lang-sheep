mod env;
mod ast;
mod parser;
mod errors;
mod types;
mod typechecker;
mod asm_generator;
use std::collections::HashMap;
use errors::LangSheepError;

fn main() -> Result<(), LangSheepError> {
	let source = "(print \"faith\")";
	let node = parser::parse(source.to_string())?;
	let mut values: HashMap<String, types::Type> = HashMap::new();
	values.insert(
		"print".to_string(), 
		types::Type::Function(
			vec![types::Type::Simple("String".to_string())],
			Box::new(types::Type::Empty)));
	let env: env::Env<types::Type> = env::Env::top(values);
	let typed_node = typechecker::typecheck(node, env)?;
	asm_generator::generate_asm(typed_node.clone(), "program.asm".to_string())?;
    println!("{:?}", typed_node);
    Ok(())
}

