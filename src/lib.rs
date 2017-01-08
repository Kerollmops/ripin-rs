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
//! # Expression Usage
//!
//! Let's define an expression:
//!
//! ```rust
//! let expr = "3 4 + 2 *"; // (3 + 4) * 2
//! ```
//!
//! Tokenize it:
//!
//! ```rust
//! # let expr = "3 4 + 2 *";
//! let tokens = expr.split_whitespace();
//! ```
//!
//! Ripin can evaluate `floating-point` expressions:
//!
//! ```rust
//! # let expr = "3 4 + 2 *";
//! # let tokens = expr.split_whitespace();
//! use ripin::evaluate::FloatExpr;
//!
//! let expr = FloatExpr::<f32>::from_iter(tokens).unwrap();
//! assert_eq!(expr.evaluate(), Ok(14.0));
//! ```
//!
//! like `integers` ones:
//!
//! ```rust
//! # let expr = "3 4 + 2 *";
//! # let tokens = expr.split_whitespace();
//! use ripin::evaluate::IntExpr;
//!
//! let expr = IntExpr::<i32>::from_iter(tokens).unwrap();
//! assert_eq!(expr.evaluate(), Ok(14));
//! ```
//!
//! # Variable Expression Usage
//!
//! You need variable expressions ?
//! No problem Ripin can do this too !
//!
//! Declare some variables:
//!
//! ```rust
//! let variables = vec![3.0, 500.0];
//! ```
//!
//! Once variables as been set, do the same as before:
//!
//! ```rust
//! # let variables = vec![3.0, 500.0];
//! use ripin::evaluate::VariableFloatExpr;
//! use ripin::variable::VarIdx;
//!
//! let expr = "3 4 + 2 * $0 -"; // (3 + 4) * 2 - $0
//!
//! let tokens = expr.split_whitespace();
//! let expr = VariableFloatExpr::<f32, VarIdx>::from_iter(tokens).unwrap();
//! ```
//!
//! Evaluate the expression with informations about the way of indexation (`usize`):
//!
//! ```rust
//! # let variables = vec![3.0, 500.0];
//! # use ripin::evaluate::VariableFloatExpr;
//! # use ripin::variable::VarIdx;
//! # let expr = "3 4 + 2 * $0 -"; // (3 + 4) * 2 - $0
//! # let tokens = expr.split_whitespace();
//! # let expr = VariableFloatExpr::<f32, VarIdx>::from_iter(tokens).unwrap();
//! assert_eq!(expr.evaluate_with_variables::<usize, _>(&variables), Ok(11.0));
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

/// TryFrom/Into_ref conversion module
pub mod convert_ref;

/// Operation on expressions and `Expression` construction methods.
pub mod expression;

/// Useful structs to use variables with expressions
pub mod variable;

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
