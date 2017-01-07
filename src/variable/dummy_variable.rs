use std::fmt;
use convert_ref::TryFromRef;

/// Struct that implement [`TryFromRef`] and always returns `Err`,
/// used to fake variables when don't needed in expressions.
///
/// Prefer using this type with the [`DummyVariables`] fake container.
///
/// [`TryFromRef`]: ../convert_ref/trait.TryFromRef.html
/// [`DummyVariables`]: ../variable/struct.DummyVariables.html
#[derive(Debug, Copy, Clone)]
pub struct DummyVariable;

impl<T> TryFromRef<T> for DummyVariable {
    type Err = ();

    fn try_from_ref(_: &T) -> Result<Self, Self::Err> {
        Err(())
    }
}

impl fmt::Display for DummyVariable {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Err(fmt::Error)
    }
}
