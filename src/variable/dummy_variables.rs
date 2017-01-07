use std::ops::Index;
use std::default::Default;
use std::marker::PhantomData;

/// Struct that implement [`Index`],
/// used to fake variables when don't needed in expressions.
///
/// Prefer using this container with the [`DummyVariable`] fake type.
///
/// # Panics
/// Panics if you call the [`index()`] method.
///
/// [`Index`]: https://doc.rust-lang.org/std/ops/trait.Index.html
/// [`DummyVariable`]: ../variable/struct.DummyVariable.html
/// [`index()`]: https://doc.rust-lang.org/std/ops/trait.Index.html#tymethod.index
#[derive(Debug)] // TODO Default here
pub struct DummyVariables<T>(PhantomData<T>);

impl<T> Default for DummyVariables<T> {
    fn default() -> Self {
        DummyVariables(PhantomData::default())
    }
}

impl<T> Index<()> for DummyVariables<T> {
    type Output = T;

    fn index(&self, _: ()) -> &Self::Output {
        panic!("DummyVariables cannot return variable!")
    }
}
