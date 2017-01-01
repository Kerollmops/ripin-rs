use std::iter::IntoIterator;

pub trait TryFromIterator<A>: Sized {
    type Err;

    fn try_from_iter<T>(iter: T) -> Result<Self, Self::Err> where T: IntoIterator<Item=A>;
}
