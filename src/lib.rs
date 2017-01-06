//! This library provide a way to evaluate [`Reverse Polish Notated`] expressions for
//! numbers and basic evaluators.
//!
//! [`Floats`] and [`signed integers`] has already been implemented with basic evaluators
//! (cf. `+`, `-`, `*`, `/`, `neg`, `pow`...).
//!
//! Of course you can build your own Evaluators by implementing the [`Evaluate`] Trait,
//! the only constraint is the compatibility with the [`Operand`] type,
//! that can be everything you want (cf. `letters`, `Enums`, `chinese symbols`...).
//!
//! # Example
//!
//! ```
//! use ripin::evaluate::{SimpleFloatExpr, SimpleIntExpr};
//!
//! let str_expr = "3 4 + 2 *"; // (3 + 4) * 2
//! let tokens = str_expr.split_whitespace();
//! let expr = SimpleFloatExpr::<f32>::from_iter(tokens).unwrap();
//!
//! assert_eq!(expr.evaluate(), Ok(14.0)); // yup that's a Float evaluation
//!
//! // let's try an Integer evaluation:
//! let tokens = str_expr.split_whitespace();
//! let expr = SimpleIntExpr::<i32>::from_iter(tokens).unwrap();
//! assert_eq!(expr.evaluate(), Ok(14));
//! ```
//!
//! [`Reverse Polish Notated`]: https://en.wikipedia.org/wiki/Reverse_Polish_notation
//! [`str`]: https://doc.rust-lang.org/std/str/index.html
//! [`Floats`]: evaluate/enum.FloatEvaluator.html
//! [`signed integers`]: evaluate/enum.IntEvaluator.html
//! [`Evaluate`]: evaluate/trait.Evaluate.html
//! [`Operand`]: expression/enum.Arithm.html

extern crate num;

mod stack;

/// Custom conversion module
pub mod convert_ref;

/// Operation on expressions and `Expression` construction methods.
pub mod expression;

/// `Evaluate Trait` and default `Evaluators`.
pub mod evaluate;

pub use stack::Stack;

/// Removes the last two elements from a stack and return them,
/// or `None` if there is not enough element.
pub fn pop_two_operands<T>(stack: &mut Stack<T>) -> Option<(T, T)> {
    if stack.len() >= 2 {
        let (a, b) = (stack.pop().unwrap(), stack.pop().unwrap());
        Some((b, a))
    } else {
        None
    }
}
