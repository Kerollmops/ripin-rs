use stack::Stack;

mod float;
mod integer;

pub use self::float::{FloatOperator, FloatErr, FloatOperateErr};
pub use self::integer::{IntOperator, IntErr, IntOperateErr};

/// The main `Trait` allowing operations on [`Operands`].
///
/// [`Operands`]: ../expression/enum.Arithm.html
pub trait Operate<T> {
    /// The type returned in the event of an operation error.
    type Err;

    /// Returns the number of operand this `Operator` needs
    /// and will `pop()` from the `stack`.
    fn operands_needed(&self) -> usize;

    /// Returns the number of operand this `Operator` will generate
    /// and will `push()` in the `stack`.
    fn operands_generated(&self) -> usize;

    /// Execute the operator on the given `stack`,
    /// returns the `Operation` error if something goes wrong.
    fn operate(self, stack: &mut Stack<T>) -> Result<(), Self::Err>;
}
