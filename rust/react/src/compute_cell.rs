use crate::{ComputeCellID, CellID, Cell};
use std::rc::Rc;
use std::borrow::Borrow;

pub(crate) struct ComputeCell<'a, T> {
	pub value: T,
	parents: Vec<&'a ComputeCell<'a, T>>,
	dependents: Vec<&'a mut dyn Cell<'a, T>>,
	closure: Box<dyn Fn(&[T]) -> T>
}

impl <'a, T: Copy + Clone> Cell<'a, T> for ComputeCell<'a, T> {
	fn get_value(&self) -> T {
		self.value
	}

	fn add_parent(&mut self, cc: &'a ComputeCell<'a, T>) {
		self.parents.push(cc);
	}
}

impl <'a, T: Copy + Clone> ComputeCell<'a, T> {
	pub fn new<F: Fn(&[T]) -> T + 'static>(value: T, dependents: Vec<&'a mut dyn Cell<'a, T>>, closure: F) -> Self {
		let mut cc = Self {
			value,
			parents: vec![],
			dependents,
			closure: Box::new(closure),
		};

		for dep in cc.dependents.iter_mut() {
			//dep.add_parent(&cc);
		}

		cc
	}

	pub fn calculate(&mut self) {
		/*for dep in self.dependents {
			match dep {
				CellID::Input(input_cell_id) => {
					let v = self.input_cell_map.get(input_cell_id).cloned().unwrap(); //sj_todo danger
					values.push(v.value());
				}
				CellID::Compute(compute_cell_id) => {
					let v = self.compute_cell_map.get(compute_cell_id).cloned().unwrap(); //sj_todo danger
					values.push(v.value());
				}
			}
		}*/
	}
}

#[cfg(test)]
mod tests {
	use std::rc::Rc;
	use crate::compute_cell::ComputeCell;
	use crate::input_cell::InputCell;

	#[test]
	fn it_works() {
		let mut ic1 = InputCell::new(1);
		let mut cc1 = ComputeCell::new(1, vec![&mut ic1], |v| v[0] + 1);
		let cc2 = ComputeCell::new(2, vec![&mut ic1, &mut cc1], |v| v[0] + 1);
	}
}