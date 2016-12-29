#![feature(try_from)]

extern crate num;

pub mod stack;
pub mod expression;

pub use stack::Stack;

pub fn pop_two_operands<T>(stack: &mut Stack<T>) -> Option<(T, T)> {
    match (stack.pop(), stack.pop()) {
        (Some(a), Some(b)) => Some((a, b)),
        _ => None,
    }
}
