use stack::Stack;
use expression::Expression;
use variable::DummyVariable;

mod float;
mod integer;

pub use self::float::{FloatEvaluator, FloatErr, FloatEvaluateErr};
pub use self::integer::{IntEvaluator, IntErr, IntEvaluateErr};

/// An helping alias to make [`Float Expressions`](enum.FloatEvaluator.html).
pub type FloatExpr<T> = Expression<T, DummyVariable, FloatEvaluator<T>>;

/// An helping alias to make [`Integer Expressions`](enum.IntEvaluator.html).
pub type IntExpr<T> = Expression<T, DummyVariable, IntEvaluator<T>>;

/// An helping alias to make variable [`Float Expressions`](enum.FloatEvaluator.html).
pub type VariableFloatExpr<T, V> = Expression<T, V, FloatEvaluator<T>>;

/// An helping alias to make variable [`Integer Expressions`](enum.IntEvaluator.html).
pub type VariableIntExpr<T, V> = Expression<T, V, IntEvaluator<T>>;

/// The main `Trait` allowing evaluation of operations on [`Operands`].
///
/// [`Operands`]: ../expression/enum.Arithm.html
pub trait Evaluate<T> {
    /// The type returned in the event of an evaluation error.
    type Err;

    /// Returns the number of operand this `Evaluator` needs
    /// and will `pop()` from the `stack`.
    fn operands_needed(&self) -> usize;

    /// Returns the number of operand this `Evaluator` will generate
    /// and will `push()` in the `stack`.
    fn operands_generated(&self) -> usize;

    /// Execute the evaluation with the given `stack`,
    /// returns the `Evaluation` error if something goes wrong.
    fn evaluate(self, stack: &mut Stack<T>) -> Result<(), Self::Err>;
}
