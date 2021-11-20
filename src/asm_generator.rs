use crate::ast::{Node, NodeFields};
use crate::env::Env;
use crate::types::Type;
use crate::LangSheepError;

fn generate_asm_print(arg: Node, text: &mut String, asm: &mut String) -> Result<(), LangSheepError> {
	// TODO general function call
	// let mut instruction = "".to_string();
	// if is_pointer_type(arg.lang_type) {
	// instruction = "leaq".to_string();
	// } else {
		// instruction = "movq".to_string();
	// }
	asm.push_str("\tleaq\t");
	generate_asm_node(arg, text, asm)?;
	asm.push_str(", %rdi\n");
	// asm.push_str("\tcall")
	Ok(())
}

fn generate_asm_node(node: Node, text: &mut String, asm: &mut String) -> Result<(), LangSheepError> {
	println!("generate_asm_node {:?}", node);
	match node.node {
		NodeFields::Call(function, args) => {
			if let NodeFields::Name(name) = function.node {
				if name == "print".to_string() {
					println!("generate print");
					if args.len() == 1 {
						return generate_asm_print(args[0].clone(), text, asm);
					} else {
						return Err(LangSheepError::GeneratorError);
					}
				}
			} else {
				return Err(LangSheepError::GeneratorError);
			}
		},
		NodeFields::String(string_text) => {
			text.push_str(&format!("\t.string \"{}\"\n", string_text));
			text.push_str("\t.text\n");
			asm.push_str(&format!("\t.text_section(%rip)"));
			return Ok(());
		}
		_ => {
			unimplemented!();
		}
	}
	Err(LangSheepError::GeneratorError)
}

pub fn generate_asm(typed_node: Node, path: String) -> Result<(), LangSheepError> {
	match typed_node.node {
		NodeFields::Program(items) => {
			let mut text = "".to_string();
			let mut asm = "".to_string();
			for item in items {
				generate_asm_node(item, &mut text, &mut asm)?;
			}
			println!("{:?}", text);
			println!("{:?}", asm);
			Ok(())
		},
		_ => {
			// not supported non-program
			Err(LangSheepError::GeneratorError)
		}
	}
}
