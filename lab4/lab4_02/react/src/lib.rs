use std::collections::{HashMap, HashSet};
use std::ops::Deref;

/// `InputCellId` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InputCellId(usize);
/// `ComputeCellId` is a unique identifier for a compute cell.
/// Values of type `InputCellId` and `ComputeCellId` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellId = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellId = r.create_compute(&[react::CellId::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComputeCellId(usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CallbackId(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

#[derive(Debug, PartialEq, Eq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

pub struct Reactor<T> {
    inputs: Vec<T>,
    cells: Vec<ComputeCell<T>>,
    inv_deps: HashMap<CellId, HashSet<ComputeCellId>>,
    // cellid -> dep to compute only dirty cells
    cbcells: HashMap<CallbackId, ComputeCellId>,
    cb_ids: usize
}

struct ComputeCell<T> {
    val: Option<T>,
    deps: Vec<CellId>,
    fun: Box<dyn Fn(&[T]) -> T>,
    callbacks: HashMap<CallbackId, Box<dyn FnMut(T)>>
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<T: Copy + PartialEq> Reactor<T> {
    pub fn new() -> Self {
        Self {
            inputs: vec![],
            cells: vec![],
            inv_deps: Default::default(),
            cbcells: Default::default(),
            cb_ids: 0
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellId {
        self.inputs.push(initial);
        InputCellId(self.inputs.len() - 1)
    }

    /// Retrieves the current values of the cells.
    /// The ids corresponding to non-existing cells are ignored
    pub fn values(&self, cell_ids: &[CellId]) -> Vec<T> {
        let mut values_vec:Vec<T> = vec![];

        for id in cell_ids {
            let v = match id {
                CellId::Input(id) => Some(self.inputs[id.0]),
                CellId::Compute(id) => self.cells[id.0].val
            };

            if v.is_some() { values_vec.push(v.unwrap()) }
        }

        values_vec
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
    pub fn create_compute<'a, F: 'a + Fn(&[T]) -> T>(
        &mut self,
        dependencies: &[CellId],
        compute_func: F,
    ) -> Result<ComputeCellId, CellId> {
        for d in dependencies {
            match d {
                CellId::Input(id) => {
                    if id.0 >= self.inputs.len() {   // input cell does not exist (since they are not removable)
                        return Err(CellId::Input(InputCellId(id.0)));
                    }
                },
                CellId::Compute(id) => {
                    if id.0 >= self.cells.len() {
                        return Err(CellId::Compute(ComputeCellId(id.0)));
                    }
                }
            }
        }

        let mut compute_cell = ComputeCell {
            val: None,
            deps: Vec::from(dependencies),
            fun: Box::new(compute_func),
            callbacks: Default::default()
        };

        compute_cell.val = Some(compute_cell.fun.deref()((self.values(&compute_cell.deps).as_slice())));
        self.cells.push(compute_cell);
        Ok(ComputeCellId(self.cells.len()-1))
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellId) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellId) -> Option<T> {
        return match id {
            CellId::Input(id) => Some(self.inputs[id.0]),
            _ => None
        }
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellId) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellId, new_value: T) -> bool {
        if id.0 < self.inputs.len() {
            if self.inputs[id.0] != new_value {
                self.inputs[id.0] = new_value;
            }
            return true;
        }
        false
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
        _id: ComputeCellId,
        _callback: F,
    ) -> Option<CallbackId> {
        unimplemented!()
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellId,
        callback: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        unimplemented!(
            "Remove the callback identified by the CallbackId {:?} from the cell {:?}",
            callback,
            cell,
        )
    }
}
