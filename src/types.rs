
// use std::collections::HashMap;


#[derive(Debug, Clone, PartialEq)]
pub enum Type {
	Simple(String),
	Function(Vec<Type>, Box<Type>),
	Empty,
	Unknown
}
