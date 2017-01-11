use std::convert::From;
use std::str::FromStr;
use convert_ref::TryFromRef;

/// Default variable keeping an index on an [`Indexable`] variable container.
/// Like a [`Vec`] or an [`HashMap`] for example.
///
/// [`Indexable`]: https://doc.rust-lang.org/std/ops/trait.Index.html
/// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
/// [`hashMap`]: https://doc.rust-lang.org/nightly/std/collections/struct.HashMap.html
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct IndexVar(usize);

#[derive(Debug)]
pub enum VarIdxErr<'a, E> {
    InvalidVariableName(&'a str),
    ConvertErr(E),
}

impl<'a> TryFromRef<&'a str> for IndexVar {
    type Err = VarIdxErr<'a, <usize as FromStr>::Err>;

    fn try_from_ref(s: &&'a str) -> Result<Self, Self::Err> {
        match s.chars().next() {
            Some('$') => {
                match FromStr::from_str(&s[1..]) {
                    Ok(n) => Ok(IndexVar(n)),
                    Err(err) => Err(VarIdxErr::ConvertErr(err)),
                }
            },
            _ => Err(VarIdxErr::InvalidVariableName(s)),
        }
    }
}

impl From<IndexVar> for usize {
    fn from(var_idx: IndexVar) -> Self {
        var_idx.0
    }
}
