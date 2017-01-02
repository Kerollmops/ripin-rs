//! This library provide a way to evaluate [`Reverse Polish Notated`] expressions for
//! numbers and basic operators.
//!
//! [`Floats`] and [`signed integers`] has already been implemented with basic operators
//! (cf. `+`, `-`, `*`, `/`, `neg`, `pow`...).
//!
//! Of course you can build your own Operators by implementing the [`Operate`] Trait,
//! the only constraint is the compatibility with the [`Operand`] type,
//! that can be everything you want (cf. `letters`, `Enums`, `chinese symbols`...).
//!
//! # Example
//!
//! ```
//! use ripin::TryFromIterator;
//! use ripin::operate::{FloatExpression, IntExpression};
//!
//! let str_expr = "3 4 + 2 *"; // (3 + 4) * 2
//! let tokens = str_expr.split_whitespace();
//! let expr: FloatExpression<f32> = TryFromIterator::try_from_iter(tokens).unwrap();
//!
//! assert_eq!(expr.operate(), Ok(14.0)); // yup that's a Float evaluation
//!
//! // let's try an Integer evaluation:
//! let tokens = str_expr.split_whitespace();
//! let expr: IntExpression<i32> = TryFromIterator::try_from_iter(tokens).unwrap();
//! assert_eq!(expr.operate(), Ok(14));
//! ```
//!
//! [`Reverse Polish Notated`]: https://en.wikipedia.org/wiki/Reverse_Polish_notation
//! [`str`]: https://doc.rust-lang.org/std/str/index.html
//! [`Floats`]: operate/enum.FloatOperator.html
//! [`signed integers`]: operate/enum.IntOperator.html
//! [`Operate`]: operate/trait.Operate.html
//! [`Operand`]: expression/enum.Arithm.html

extern crate num;

mod stack;
mod try_from_iterator;
mod try_from_ref;

/// Operation on expressions and `Expression` construction methods.
pub mod expression;
/// `Operate Trait` and default `Operators`.
pub mod operate;

pub use stack::Stack;
pub use try_from_iterator::TryFromIterator;
pub use try_from_ref::TryFromRef;

/// Removes the last two elements from a stack and returns it,
/// or `None` if it is empty.
pub fn pop_two_operands<T>(stack: &mut Stack<T>) -> Option<(T, T)> {
    match (stack.pop(), stack.pop()) {
        (Some(a), Some(b)) => Some((b, a)),
        _ => None,
    }
}
