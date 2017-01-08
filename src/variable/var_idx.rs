use std::convert::From;
use std::str::FromStr;
use convert_ref::TryFromRef;

/// Default variable keeping an index on an [`Indexable`] variable container.
/// Like a [`Vec`] for example.
///
/// [`Indexable`]: https://doc.rust-lang.org/std/ops/trait.Index.html
/// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
#[derive(Copy, Clone)]
pub struct VarIdx(usize);

#[derive(Debug)]
pub enum VarIdxErr<'a, E> {
    InvalidVariableName(&'a str),
    ConvertErr(E),
}

impl<'a> TryFromRef<&'a str> for VarIdx {
    type Err = VarIdxErr<'a, <usize as FromStr>::Err>;

    fn try_from_ref(s: &&'a str) -> Result<Self, Self::Err> {
        match s.chars().next() {
            Some('$') => {
                match FromStr::from_str(&s[1..]) {
                    Ok(n) => Ok(VarIdx(n)),
                    Err(err) => Err(VarIdxErr::ConvertErr(err)),
                }
            },
            _ => Err(VarIdxErr::InvalidVariableName(s)),
        }
    }
}

impl From<VarIdx> for usize {
    fn from(var_idx: VarIdx) -> Self {
        var_idx.0
    }
}
