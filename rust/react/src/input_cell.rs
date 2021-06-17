use crate::compute_cell::ComputeCell;
use crate::Cell;

pub struct InputCell<'a, T> {
	pub value: T,
	parents: Vec<&'a ComputeCell<'a, T>>,
}

impl <'a, T: Copy + Clone> Cell<'a, T> for InputCell<'a, T> {
	fn get_value(&self) -> T {
		self.value
	}

	fn add_parent(&mut self, cc: &'a ComputeCell<'a, T>) {
		self.parents.push(cc);
	}
}

impl <'a, T: Copy + Clone> InputCell<'a , T> {
	pub fn new(value: T) -> Self {
		Self {
			value,
			parents: vec![],
		}
	}
}