use std::default::Default;
use std::marker::PhantomData;
use variable::GetVariable;

/// Struct that implement [`Index`],
/// used to fake variables when don't needed in expressions.
///
/// Prefer using this container with the [`DummyVariable`] fake type.
///
/// [`Index`]: https://doc.rust-lang.org/std/ops/trait.Index.html
/// [`DummyVariable`]: ../variable/struct.DummyVariable.html
/// [`index()`]: https://doc.rust-lang.org/std/ops/trait.Index.html#tymethod.index
#[derive(Debug)]
pub struct DummyVariables<T>(PhantomData<T>);

impl<T> Default for DummyVariables<T> {
    fn default() -> Self {
        DummyVariables(PhantomData::default())
    }
}

impl<T> GetVariable<()> for DummyVariables<T> {
    type Output = T;

    fn get_variable(&self, _: ()) -> Option<&Self::Output> {
        None
    }
}
