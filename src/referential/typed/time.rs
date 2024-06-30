use crate::{borrowed::UnvalidatedTimeLogline as BorrowedLine, shared::validate_line, types::*};

/// A (thread safe) line string
///
/// We use a [`Arc<str>`] over [`String`] to communicate its immutability and fixedness.
/// (`Arc<str>`/`Box<str>`/`&str` don't carry any capacity data, only the length.)
///
/// While `Box<str>` is also possible, the box is not thread safe and
/// and both types have pretty similar performance characteristics.
pub type LineStr = Arc<str>;

self_cell::self_cell!(
    struct Container {
        owner: LineStr,

        #[covariant]
        dependent: BorrowedLine,
    }

    impl {Debug, PartialEq}
);

impl Clone for Container {
    fn clone(&self) -> Self {
        let input = Arc::clone(self.borrow_owner());
        Self::new(input, |line| {
            let dependent = BorrowedLine::try_from(line.as_ref())
                .expect("invalid line input despite validation");
            dependent
        })
    }
}

pub type UnvalidatedLogline = Logline<Unvalidated>;
pub type ValidatedLogline = Logline<Validated>;

#[derive(Debug, Clone, PartialEq)]
pub struct Logline<V> {
    inner: Container,
    _marker: PhantomData<V>,
}

impl<V> Logline<V> {
    pub fn view(&self) -> &BorrowedLine<'_> {
        self.inner.borrow_dependent()
    }

    pub fn as_raw(&self) -> &str {
        self.inner.borrow_owner()
    }

    pub fn into_raw(self) -> LineStr {
        self.inner.into_owner()
    }
}

macro_rules! impl_try_from {
    ($in:ty, $out_v:ident, $out_u:ident) => {
        impl TryFrom<$in> for $out_v {
            type Error = &'static str;

            fn try_from(line: $in) -> Result<Self, Self::Error> {
                let line: LineStr = line.into();
                validate_line(&line)?;
                let container = Container::new(line, |line| {
                    BorrowedLine::try_from(line.as_ref())
                        .expect("invalid line input despite validation")
                });
                Ok($out_v {
                    inner: container,
                    _marker: PhantomData,
                })
            }
        }

        impl TryFrom<$in> for $out_u {
            type Error = &'static str;

            fn try_from(line: $in) -> Result<Self, Self::Error> {
                let container = Container::new(line.into(), |line| {
                    BorrowedLine::try_from(line.as_ref())
                        .expect("invalid line input despite validation")
                });
                Ok($out_u {
                    inner: container,
                    _marker: PhantomData,
                })
            }
        }
    };
}

impl_try_from!(&str, ValidatedLogline, UnvalidatedLogline);
impl_try_from!(String, ValidatedLogline, UnvalidatedLogline);
impl_try_from!(Box<str>, ValidatedLogline, UnvalidatedLogline);
impl_try_from!(Arc<str>, ValidatedLogline, UnvalidatedLogline);
