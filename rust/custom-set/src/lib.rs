#[derive(Debug)]
pub struct CustomSet<T: Copy + Eq + Ord> {
	internal: Vec<T>
}

impl<T: Copy + Eq + Ord> PartialEq for CustomSet<T> {
	fn eq(&self, other: &Self) -> bool {
		self.internal.len() == other.internal.len() && self.internal.iter().all(|k| other.internal.contains(k))
	}
}

impl<T: Copy + Eq + Ord> CustomSet<T> {
	pub fn new(input: &[T]) -> Self {
		let mut internal = input.to_vec();
		internal.sort();
		internal.dedup();
		CustomSet {
			internal
		}
	}

	pub fn contains(&self, element: &T) -> bool {
		self.internal.contains(element)
	}

	pub fn add(&mut self, e: T) {
		if !self.internal.contains(&e) {
			self.internal.push(e);
		}
	}

	pub fn is_subset(&self, other: &Self) -> bool {
		self.internal.iter().all(|k| other.internal.contains(k))
	}

	pub fn is_empty(&self) -> bool {
		self.internal.is_empty()
	}

	pub fn is_disjoint(&self, other: &Self) -> bool {
		!self.internal.iter().any(|k| other.internal.contains(k))
	}

	pub fn intersection(&self, other: &Self) -> Self {
		let mut intersection: Vec<T> = self.internal.iter().filter(|e| other.internal.contains(*e)).cloned().collect();

		CustomSet {
			internal: intersection
		}
	}

	pub fn difference(&self, other: &Self) -> Self {
		let mut intersection: Vec<T> = self.internal.iter().filter(|e| !other.internal.contains(*e)).cloned().collect();

		CustomSet {
			internal: intersection
		}
	}

	pub fn union(&self, other: &Self) -> Self {
		let internal: Vec<T> = self.internal.iter().chain(other.internal.iter()).cloned().collect();
		CustomSet::new(&internal)
	}
}
