use crate::second_attempt::compute_cell::ComputeCell;
use crate::second_attempt::input_cell::InputCell;
use std::marker::PhantomData;

mod compute_cell;
mod input_cell;

/// `InputCellID` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputCellID(usize);

/// `ComputeCellID` is a unique identifier for a compute cell.
/// Values of type `InputCellID` and `ComputeCellID` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellID = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellID = r.create_compute(&[react::CellID::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComputeCellID(usize);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellID {
    Input(InputCellID),
    Compute(ComputeCellID),
}

impl CellID {
    pub fn get_id(&self) -> usize {
        match self {
            CellID::Input(ic) => ic.0,
            CellID::Compute(cc) => cc.0,
        }
    }
}

trait Cell {
    type T: Copy + PartialEq;

    fn get_value(&self) -> Self::T;

    fn get_id(&self) -> usize;

    fn compute_cell_id(&self) -> CellID;

    fn add_parent(&mut self, parent_id: CellID);
}

pub enum CellTypes<T> {
    InputCell(InputCell<T>),
    ComputeCell(ComputeCell<T>),
}

impl<T: Copy + PartialEq> CellTypes<T> {
    pub fn get_value(&self) -> T {
        match self {
            CellTypes::InputCell(ic) => ic.get_value(),
            CellTypes::ComputeCell(cc) => cc.get_value(),
        }
    }
}

pub struct Reactor<T> {
    big_vec: Vec<CellTypes<T>>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<T: Copy + PartialEq> Reactor<T> {
    pub fn new() -> Self {
        Reactor {
            big_vec: Vec::new(),
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellID {
        InputCell::new(initial, &mut self.big_vec)
    }

    pub fn create_compute<F: Fn(&[T]) -> T>(
        &mut self,
        dependencies: &[CellID],
        compute_func: F,
    ) -> Result<ComputeCellID, CellID> {
        ComputeCell::new(dependencies, compute_func, &mut self.big_vec)
    }

    pub fn value(&self, id: CellID) -> Option<T> {
        if let Some(cell) = self.big_vec.get(id.get_id()) {
            Some(cell.get_value())
        } else {
            None
        }
    }
}
