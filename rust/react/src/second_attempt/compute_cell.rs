use crate::get_id;
use crate::second_attempt::{Cell, CellID, CellTypes, ComputeCellID};

pub struct ComputeCell<T> {
    id: usize,
    value: T,
    deps: Vec<usize>,
    parents: Vec<usize>,
}

impl<T: Copy + PartialEq> ComputeCell<T> {
    pub fn new<F: Fn(&[T]) -> T>(
        deps: &[CellID],
        f: F,
        reactor: &mut Vec<CellTypes<T>>,
    ) -> Result<ComputeCellID, CellID> {
        let mut dep_ids = vec![];
        let mut dep_values = vec![];
        let mut is_valid = None;

        for e in deps.iter() {
            if let Some(d) = reactor.get(e.get_id()) {
                dep_ids.push(e.get_id());
                let v = match d {
                    CellTypes::InputCell(ic) => ic.get_value(),
                    CellTypes::ComputeCell(cc) => cc.get_value(),
                };
                dep_values.push(v);
            } else {
                is_valid = Some(*e);
                break;
            }
        }

        if is_valid.is_some() {
            Err(is_valid.unwrap())
        } else {
            let value = f(&dep_values);

            let mut compute_cell = Self {
                id: get_id(),
                value,
                deps: dep_ids,
                parents: vec![],
            };

            let id = ComputeCellID(compute_cell.get_id());

            for e in deps.iter() {
                if let Some(d) = reactor.get_mut(e.get_id()) {
                    match d {
                        CellTypes::InputCell(ic) => {
                            ic.add_parent(CellID::Compute(id));
                        }
                        CellTypes::ComputeCell(cc) => {
                            cc.add_parent(CellID::Compute(id));
                        }
                    };
                }
            }

            reactor.push(CellTypes::ComputeCell(compute_cell));
            Ok(id)
        }
    }
}

impl<V: Copy + PartialEq> Cell for ComputeCell<V> {
    type T = V;

    fn get_value(&self) -> Self::T {
        self.value
    }

    fn get_id(&self) -> usize {
        self.id
    }

    fn compute_cell_id(&self) -> CellID {
        CellID::Compute(ComputeCellID(self.get_id()))
    }

    fn add_parent(&mut self, parent_id: CellID) {
        self.parents.push(parent_id.get_id());
    }
}

#[cfg(test)]
mod tests {
    use crate::second_attempt::compute_cell::ComputeCell;
    use crate::second_attempt::input_cell::InputCell;
    use crate::second_attempt::{Cell, CellID, InputCellID};

    #[test]
    fn it_works() {
        let mut big_vec = vec![];

        let mut ic1: dyn Cell = InputCell::new(1);
        let mut cc1 = ComputeCell::new(1, &vec![ic1.compute_cell_id()]);
        /*let cc2 = ComputeCell::new(2, &vec![cc1.compute_cell_id(), ic1.compute_cell_id()]);

        // ic1 will have two parents
        ic1.add_parent(cc1.compute_cell_id());
        ic1.add_parent(cc2.compute_cell_id());

        // cc1 will have only one parent
        cc1.add_parent(cc2.compute_cell_id());*/
    }
}
