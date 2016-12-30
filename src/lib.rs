#![feature(try_from)]

extern crate num;

mod stack;
pub mod expression;
pub mod operate;

pub fn pop_two_operands<T>(stack: &mut stack::Stack<T>) -> Option<(T, T)> {
    match (stack.pop(), stack.pop()) {
        (Some(a), Some(b)) => Some((a, b)),
        _ => None,
    }
}
