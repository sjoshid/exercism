mod compute_cell;
mod input_cell;
mod second_attempt;

use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::path::Prefix::Verbatim;
use std::sync::atomic::{AtomicUsize, Ordering};

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
pub struct CallbackID();

pub trait Cell<'a, T> {
    fn get_value(&self) -> T;
    fn add_parent(&mut self, cc: &'a compute_cell::ComputeCell<'a, T>);
}

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

#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

// Shameless copy from Rust forum. https://users.rust-lang.org/t/idiomatic-rust-way-to-generate-unique-id/33805/6
fn get_id() -> usize {
    static COUNTER: AtomicUsize = AtomicUsize::new(1); //static is crazy.
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

#[derive(Clone, Debug)]
struct ComputeCell<T> {
    value: T,
    dependents: Option<Vec<ComputeCellID>>,
}

impl<T> ComputeCell<T>
where
    T: Copy + Clone,
{
    pub fn new(value: T) -> (ComputeCellID, Self) {
        (
            ComputeCellID(get_id()),
            ComputeCell {
                value,
                dependents: None,
            },
        )
    }

    pub fn add_dep(&mut self, id: ComputeCellID) {
        if let Some(d) = self.dependents.as_mut() {
            d.push(id);
        } else {
            let mut d = Vec::new();
            d.push(id);
            self.dependents = Some(d);
        }
    }

    pub fn value(&self) -> T {
        self.value
    }
}

#[derive(Clone, Debug)]
struct InputCell<T> {
    value: T,
    dependents: Option<Vec<ComputeCellID>>,
}

impl<T> InputCell<T>
where
    T: Copy + Clone,
{
    pub fn new(value: T) -> (InputCellID, Self) {
        (
            InputCellID(get_id()),
            InputCell {
                value,
                dependents: None,
            },
        )
    }

    pub fn value(&self) -> T {
        self.value
    }

    pub fn add_dep(&mut self, id: ComputeCellID) {
        if let Some(d) = self.dependents.as_mut() {
            d.push(id);
        } else {
            let mut d = Vec::new();
            d.push(id);
            self.dependents = Some(d);
        }
    }
}

pub struct Reactor<T> {
    input_cell_map: HashMap<InputCellID, InputCell<T>>,
    compute_cell_map: HashMap<ComputeCellID, ComputeCell<T>>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<T: Copy + PartialEq> Reactor<T> {
    pub fn new() -> Self {
        Reactor {
            input_cell_map: HashMap::new(),
            compute_cell_map: HashMap::new(),
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellID {
        let ic = InputCell::new(initial);
        self.input_cell_map.insert(ic.0, ic.1);
        ic.0
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: Fn(&[T]) -> T>(
        &mut self,
        dependencies: &[CellID],
        compute_func: F,
    ) -> Result<ComputeCellID, CellID> {
        let mut values: Vec<T> = Vec::with_capacity(dependencies.len());
        for dep in dependencies {
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
        }
        //sj_todo handle panics
        let computed_value = compute_func(values.as_ref());
        let cc = ComputeCell::new(computed_value);
        self.compute_cell_map.insert(cc.0, cc.1);

        for dep in dependencies {
            match dep {
                CellID::Input(input_cell_id) => {
                    let v = self.input_cell_map.get_mut(input_cell_id).unwrap(); //sj_todo danger
                    v.add_dep(cc.0);
                }
                CellID::Compute(compute_cell_id) => {
                    let v = self.compute_cell_map.get_mut(compute_cell_id).unwrap(); //sj_todo danger
                    v.add_dep(cc.0);
                }
            }
        }

        Ok(cc.0)
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellID) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellID) -> Option<T> {
        match id {
            CellID::Input(input_cell_id) => {
                self.input_cell_map.get(&input_cell_id).map(|v| v.value())
            }
            CellID::Compute(compute_cell_id) => self
                .compute_cell_map
                .get(&compute_cell_id)
                .map(|v| v.value()),
        }
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellID) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellID, value: T) -> bool {
        if let Some(ic) = self.input_cell_map.get_mut(&id) {
            if value != ic.value {
                ic.value = value;

                if let Some(deps) = ic.dependents.as_mut() {
                    for dep in deps {
                        if let Some(cc) = self.compute_cell_map.get_mut(dep) {}
                    }
                }
            }
        }
        todo!()
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T)>(
        &mut self,
        _id: ComputeCellID,
        _callback: F,
    ) -> Option<CallbackID> {
        unimplemented!()
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellID,
        callback: CallbackID,
    ) -> Result<(), RemoveCallbackError> {
        unimplemented!(
            "Remove the callback identified by the CallbackID {:?} from the cell {:?}",
            callback,
            cell,
        )
    }
}
