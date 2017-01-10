use std::hash::Hash;
use std::cmp::{Eq, Ord};
use std::collections::{HashMap, VecDeque, LinkedList, BTreeMap};

/// This trait allow [`Expression`] to retrieve variables
/// from an from a container using an index variable like [`IndexVar`].
///
/// [`Expression`]: ../expression/struct.Expression.html
/// [`IndexVar`]: ../variable/struct.VarIdx.html
pub trait GetVariable<I> {
    type Output;

    fn get_variable(&self, index: I) -> Option<&Self::Output>;
}

impl<I: Hash + Eq, T> GetVariable<I> for HashMap<I, T> {
    type Output = T;

    fn get_variable(&self, index: I) -> Option<&Self::Output> {
        self.get(&index)
    }
}

impl<I: Ord, T> GetVariable<I> for BTreeMap<I, T> {
    type Output = T;

    fn get_variable(&self, index: I) -> Option<&Self::Output> {
        self.get(&index)
    }
}

impl<T> GetVariable<usize> for Vec<T> {
    type Output = T;

    fn get_variable(&self, index: usize) -> Option<&Self::Output> {
        self.iter().nth(index)
    }
}

impl<T> GetVariable<usize> for VecDeque<T> {
    type Output = T;

    fn get_variable(&self, index: usize) -> Option<&Self::Output> {
        self.get(index)
    }
}

impl<T> GetVariable<usize> for LinkedList<T> {
    type Output = T;

    fn get_variable(&self, index: usize) -> Option<&Self::Output> {
        self.iter().nth(index)
    }
}
