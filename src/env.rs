use std::collections::HashMap;


#[derive(Debug, Clone)]
pub struct Env<T> {
	values: HashMap<String, T>,
	parent: Option<Box<Env<T>>>
}

impl<T> Env<T> {
	pub fn top(values: HashMap<String, T>) -> Env<T> {
		Env { values: values, parent: None }
	}

	pub fn get(&self, name: &str) -> Option<&T> {
		// let mut find_in_env: Option<Box<Env<T>>> = Some(self);
		let mut find_in_env: &Env<T> = self;
		loop {
			match find_in_env.values.get(name) {
				Some(t) => {
					return Some(t)
				},
				None => {
					match &find_in_env.parent {
						Some(boxed_env) => {
							find_in_env = boxed_env;
						},
						None => {
							return None;
						}
					}
				}
			}
		}
		// None
	} 
}

