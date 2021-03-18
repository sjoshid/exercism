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

	while si < sl.len() {
		if fl[fi] == sl[si] {
			// increment fi
			fi = fi + 1;
		} else {
			//reset fi. keep si same
			fi = 0;
		}
		if fi == fl.len() {
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
		si = si + 1;
	}

	result
}
