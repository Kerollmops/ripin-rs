/// Attempt to construct `Self` via a conversion from a ref.
pub trait TryFromRef<T>: Sized {
    /// The type returned in the event of a conversion error.
    type Err;

    /// Performs the conversion.
    fn try_from_ref(&T) -> Result<Self, Self::Err>;
}

/// An attempted conversion that don't consumes `self`, which may or may not be expensive.
///
/// Library authors should not directly implement this trait,
/// but should prefer implementing the [`TryFromRef`] trait, which offers greater flexibility
/// and provides an equivalent `TryInto` implementation for free,
/// thanks to a blanket implementation in the standard library.
///
/// [`TryFromRef`]: ../convert_ref/trait.TryFromRef.html
pub trait TryIntoRef<T>: Sized {
    /// The type returned in the event of a conversion error.
    type Err;

    /// Performs the conversion.
    fn try_into_ref(&self) -> Result<T, Self::Err>;
}

impl<T, U> TryIntoRef<U> for T
    where U: TryFromRef<T>
{
    type Err = U::Err;

    fn try_into_ref(&self) -> Result<U, U::Err> {
        U::try_from_ref(&self)
    }
}

macro_rules! implement_try_from_ref {
    ( $($x:ty) * ) => {
        use std::str::FromStr;
        $(
            impl<'a> TryFromRef<&'a str> for $x {
                type Err = <$x as FromStr>::Err;

                fn try_from_ref(s: &&'a str) -> Result<Self, Self::Err> {
                    FromStr::from_str(s)
                }
            }
        )*
    };
}

implement_try_from_ref!(f32 f64 isize i8 i16 i32 i64 usize u8 u16 u32 u64);
