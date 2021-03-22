#[derive(Debug)]
pub struct CustomSet<T: Copy + Eq  + Ord> {
	internal: Vec<T>
}

impl<T: Copy + Eq  + Ord> PartialEq for CustomSet<T> {
	fn eq(&self, other: &Self) -> bool {
		self.internal.len() == other.internal.len() && self.internal.iter().all(|k| other.internal.contains(k))
	}
}

impl<T: Copy + Eq  + Ord> CustomSet<T> {
	pub fn new(input: &[T]) -> Self {
		CustomSet {
			internal: input.to_vec()
		}
	}

	pub fn contains(&self, element: &T) -> bool {
		self.internal.binary_search(element).is_ok()
	}

	pub fn add(&mut self, e: T) {
		if self.internal.binary_search(&e).is_err() {
			self.internal.push(e);
		}
	}

	pub fn is_subset(&self, other: &Self) -> bool {
		match (self.internal.len(), other.internal.len()) {
			(0, 0) | (0, _) => {
				true
			},
			(_, 0) => {
				false
			}
			(_, _) => {
				self.internal.iter().all(|k| other.internal.contains(k))
			}
		}
	}

	pub fn is_empty(&self) -> bool {
		self.internal.is_empty()
	}

	pub fn is_disjoint(&self, other: &Self) -> bool {
		match (self.internal.len(), other.internal.len()) {
			(0, 0) | (0, _) | (_, 0) => {
				true
			},
			(_, _) => {
				!self.internal.iter().any(|k| other.internal.contains(k))
			}
		}
	}

	pub fn intersection(&self, other: &Self) -> Self {
		match (self.internal.len(), other.internal.len()) {
			(0, _) | (_, 0) => {
				CustomSet {
					internal: Vec::new()
				}
			},
			(_, _) => {
				let mut intersection = Vec::new();
				for element in self.internal.iter() {
					if other.internal.contains(element) {
						intersection.push(*element);
					}
				}
				CustomSet {
					internal: intersection
				}
			}
		}
	}

	pub fn difference(&self, other: &Self) -> Self {
		match (self.internal.len(), other.internal.len()) {
			(0, _) => {
				Self::new(&[])
			},
			(_, 0) => {
				Self::new(self.internal.as_slice())
			},
			(_, _) => {
				let mut difference = Vec::new();
				for element in self.internal.iter() {
					if !other.internal.contains(element) {
						difference.push(*element);
					}
				}
				CustomSet {
					internal: difference
				}
			}
		}
	}

	pub fn union(&self, other: &Self) -> Self {
		let mut internal = Vec::new();
		for k in self.internal.iter() {
			if !internal.contains(k) {
				internal.push(*k);
			}
		}
		for k in other.internal.iter() {
			if !internal.contains(k) {
				internal.push(*k);
			}
		}

		CustomSet {
			internal
		}
	}
}
