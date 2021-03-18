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
	let mut found = false;

	while si < sl.len() && !found {
		for (i, e) in sl[si..].iter().enumerate() {
			if fl[fi] == *e && !found {
				while fi < fl.len() && fl[fi] == sl[si] {
					fi.checked_add(1);
					si.checked_add(1);
				}

				if fi == fl.len() {
					found = true;
				} else {
					fi = 0; //reset
				}
			}
		}
	}

	if found && flipped {
		Comparison::Superlist
	} else if found && !flipped {
		Comparison::Sublist
	} else if !found {
		Comparison::Unequal
	} else {
		Comparison::Equal
	}
}
