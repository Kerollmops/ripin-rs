use stack::Stack;

mod float;

pub use self::float::{FloatOperator, FloatErr};

pub trait Operate<T> {
    type Err;
    fn operands_needed(&self) -> usize;
    fn operands_generated(&self) -> usize;
    fn operate(self, stack: &mut Stack<T>) -> Result<(), Self::Err>;
}
