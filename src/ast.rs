use crate::types::Type;

#[derive(Debug, Clone)]
pub struct Node {
	pub node: NodeFields,
	pub lang_type: Type
}

#[derive(Debug, Clone)]
pub enum NodeFields {
	Int(i64),
	String(std::string::String),
	Name(String),
	Call(Box<Node>, Vec<Node>),
	Program(Vec<Node>),
	None
}
