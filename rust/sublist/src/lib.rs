#[derive(Debug, PartialEq)]
pub enum Comparison {
	Equal,
	Sublist,
	Superlist,
	Unequal,
}

pub fn sublist<'a, T: PartialEq>(mut fl: &'a [T], mut sl: &'a [T]) -> Comparison {
	if fl.is_empty() && sl.is_empty() {
		return Comparison::Equal;
	}

	if fl.is_empty() && !sl.is_empty() {
		return Comparison::Sublist;
	}

	if !fl.is_empty() && sl.is_empty() {
		return Comparison::Superlist;
	}

	let mut flipped = false;

	if fl.len() > sl.len() {
		//flip
		let temp = sl;
		sl = fl;
		fl = temp;
		flipped = true;
	}

	let mut fi = 0;
	let mut si = 0;

	let mut result = Comparison::Unequal;

	while si <= sl.len() - fl.len() {
		if fl[0] == sl[si] {
			let ending_index = si + fl.len();
			if check_block(fl, &sl[si..ending_index]) {
				if fl.len() == sl.len() {
					result = Comparison::Equal;
				} else {
					if flipped {
						//superlist
						result = Comparison::Superlist;
					} else {
						//sublist
						result = Comparison::Sublist;
					}
				}
				break;
			}
		}

		si = si + 1;
	}

	result
}

fn check_block<T: PartialEq>(fl: &[T], slice_of_sl: &[T]) -> bool {
	let mut result = true;
	for (e1, e2) in fl.iter().zip(slice_of_sl.iter()) {
		if *e1 != *e2 {
			result = false;
			break;
		}
	}
	result
}
