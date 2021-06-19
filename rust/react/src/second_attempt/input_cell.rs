use crate::get_id;
use crate::second_attempt::{Cell, CellID, CellTypes, InputCellID};

pub struct InputCell<T> {
    id: usize,
    value: T,
    parents: Vec<usize>,
}

impl<T: Copy + PartialEq> InputCell<T> {
    pub fn new(value: T, reactor: &mut Vec<CellTypes<T>>) -> InputCellID {
        let ic = Self {
            id: get_id(),
            value,
            parents: vec![],
        };
        let id = InputCellID(ic.get_id());
        reactor.push(CellTypes::InputCell(ic));
        id
    }
}

impl<V: Copy + PartialEq> Cell for InputCell<V> {
    type T = V;

    fn get_value(&self) -> Self::T {
        self.value
    }

    fn get_id(&self) -> usize {
        self.id
    }

    fn compute_cell_id(&self) -> CellID {
        CellID::Input(InputCellID(self.get_id()))
    }

    fn add_parent(&mut self, parent_id: CellID) {
        self.parents.push(parent_id.get_id());
    }
}
