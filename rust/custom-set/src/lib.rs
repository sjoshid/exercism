use std::collections::HashMap;
use std::rc::Rc;
use std::hash::Hash;

const MY_VALUE: u32 = 1;

#[derive(Debug)]
pub struct CustomSet<T: Copy + Eq + Hash> {
	internal: HashMap<T, Rc<u32>>
}

impl<T: Copy + Eq + Hash> PartialEq for CustomSet<T> {
	fn eq(&self, other: &Self) -> bool {
		self.internal.len() == other.internal.len() && self.internal.keys().all(|k| other.internal.contains_key(k))
	}
}

impl<T: Copy + Eq + Hash> CustomSet<T> {
	pub fn new(input: &[T]) -> Self {
		let mut internal = HashMap::new();
		if !input.is_empty() {
			let rc_value = Rc::new(MY_VALUE);
			for e in input {
				internal.insert(*e, Rc::clone(&rc_value));
			}
		}
		CustomSet {
			internal
		}
	}

	pub fn contains(&self, e: &T) -> bool {
		self.internal.contains_key(e)
	}

	pub fn add(&mut self, e: T) {
		if self.internal.is_empty() {
			let rc_value = Rc::new(MY_VALUE);
			self.internal.insert(e, Rc::clone(&rc_value));
		} else {
			let first = self.internal.iter().next();
			let rc_value = first.unwrap().1;
			self.internal.insert(e, Rc::clone(&rc_value));
		}
	}

	pub fn is_subset(&self, other: &Self) -> bool {
		match (self.internal.len(), other.internal.len()) {
			(0, 0) => {
				true
			}
			(0, _) => {
				true
			}
			(_, 0) => {
				false
			}
			(_, _) => {
				self.internal.keys().all(|k| other.internal.contains_key(k))
			}
		}
	}

	pub fn is_empty(&self) -> bool {
		self.internal.is_empty()
	}

	pub fn is_disjoint(&self, _other: &Self) -> bool {
		unimplemented!();
	}

	pub fn intersection(&self, _other: &Self) -> Self {
		unimplemented!();
	}

	pub fn difference(&self, _other: &Self) -> Self {
		unimplemented!();
	}

	pub fn union(&self, other: &Self) -> Self {

		let mut internal = HashMap::new();
		for (k, v) in &self.internal {
			internal.insert(*k, Rc::clone(v));
		}
		for (k, v) in &other.internal {
			internal.insert(*k, Rc::clone(v));
		}

		CustomSet {
			internal
		}
	}
}
