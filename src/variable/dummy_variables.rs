use std::ops::Index;
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
#[derive(Debug, Copy, Clone)]
pub struct DummyVariables<T>(PhantomData<T>);

impl<I, T> Index<I> for DummyVariables<T> {
    type Output = T;

    fn index(&self, _: I) -> &Self::Output {
        panic!("DummyVariables cannot return variable!")
    }
}
