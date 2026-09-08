use crate::{borrowed::structured, shared::validate_line, types::*};

mod private {
    pub trait Sealed {}
}

/// A thread-safe immutable line string.
pub type LineStr = Arc<str>;

type SimpleLine<'a> = structured::SimpleLogline<'a, Unvalidated>;

#[cfg(any(feature = "time", feature = "chrono", feature = "jiff"))]
type TypedLine<'a> = structured::TypedLogline<'a, Unvalidated>;

self_cell::self_cell!(
    #[doc(hidden)]
    pub struct TextContainer {
        owner: LineStr,

        #[covariant]
        dependent: SimpleLine,
    }

    impl {Debug, PartialEq}
);

#[cfg(any(feature = "time", feature = "chrono", feature = "jiff"))]
self_cell::self_cell!(
    #[doc(hidden)]
    pub struct SelectedContainer {
        owner: LineStr,

        #[covariant]
        dependent: TypedLine,
    }

    impl {Debug, PartialEq}
);

#[doc(hidden)]
pub trait StructuredLine: private::Sealed + structured::DateTimeBackend + Sized {
    type Container: Clone + std::fmt::Debug + PartialEq;
    type View<'a>;

    fn new_container(line: LineStr) -> Result<Self::Container, &'static str>;
    fn as_raw(container: &Self::Container) -> &str;
    fn into_raw(container: Self::Container) -> LineStr;
    fn view(container: &Self::Container) -> &Self::View<'_>;
}

impl private::Sealed for structured::Text {}

impl Clone for TextContainer {
    fn clone(&self) -> Self {
        Self::new(Arc::clone(self.borrow_owner()), |line| {
            structured::SimpleLogline::try_from(line.as_ref())
                .expect("invalid line input despite validation")
        })
    }
}

impl StructuredLine for structured::Text {
    type Container = TextContainer;
    type View<'a> = structured::SimpleLogline<'a, Unvalidated>;

    fn new_container(line: LineStr) -> Result<Self::Container, &'static str> {
        TextContainer::try_new(line, |line| {
            structured::SimpleLogline::try_from(line.as_ref())
        })
    }

    fn as_raw(container: &Self::Container) -> &str {
        container.borrow_owner()
    }

    fn into_raw(container: Self::Container) -> LineStr {
        container.into_owner()
    }

    fn view(container: &Self::Container) -> &Self::View<'_> {
        container.borrow_dependent()
    }
}

#[cfg(any(feature = "time", feature = "chrono", feature = "jiff"))]
impl private::Sealed for structured::Selected {}

#[cfg(any(feature = "time", feature = "chrono", feature = "jiff"))]
impl Clone for SelectedContainer {
    fn clone(&self) -> Self {
        Self::new(Arc::clone(self.borrow_owner()), |line| {
            structured::TypedLogline::try_from(line.as_ref())
                .expect("invalid line input despite validation")
        })
    }
}

#[cfg(any(feature = "time", feature = "chrono", feature = "jiff"))]
impl StructuredLine for structured::Selected {
    type Container = SelectedContainer;
    type View<'a> = structured::TypedLogline<'a, Unvalidated>;

    fn new_container(line: LineStr) -> Result<Self::Container, &'static str> {
        SelectedContainer::try_new(line, |line| {
            structured::TypedLogline::try_from(line.as_ref())
        })
    }

    fn as_raw(container: &Self::Container) -> &str {
        container.borrow_owner()
    }

    fn into_raw(container: Self::Container) -> LineStr {
        container.into_owner()
    }

    fn view(container: &Self::Container) -> &Self::View<'_> {
        container.borrow_dependent()
    }
}

pub type UnvalidatedSimpleLogline = Logline<Unvalidated, structured::Text>;
pub type ValidatedSimpleLogline = Logline<Validated, structured::Text>;

#[cfg(any(feature = "time", feature = "chrono", feature = "jiff"))]
pub type UnvalidatedTypedLogline = Logline<Unvalidated, structured::Selected>;
#[cfg(any(feature = "time", feature = "chrono", feature = "jiff"))]
pub type ValidatedTypedLogline = Logline<Validated, structured::Selected>;

#[derive(Debug, Clone, PartialEq)]
pub struct Logline<V, D: StructuredLine> {
    inner: D::Container,
    _marker: PhantomData<V>,
}

impl<V, D: StructuredLine> Logline<V, D> {
    pub fn view(&self) -> &D::View<'_> {
        D::view(&self.inner)
    }

    pub fn as_raw(&self) -> &str {
        D::as_raw(&self.inner)
    }

    pub fn into_raw(self) -> LineStr {
        D::into_raw(self.inner)
    }
}

fn new_logline<V, D: StructuredLine>(line: LineStr) -> Result<Logline<V, D>, &'static str> {
    Ok(Logline {
        inner: D::new_container(line)?,
        _marker: PhantomData,
    })
}

macro_rules! impl_try_from {
    ($in:ty) => {
        impl<D: StructuredLine> TryFrom<$in> for Logline<Validated, D> {
            type Error = &'static str;

            fn try_from(line: $in) -> Result<Self, Self::Error> {
                let line: LineStr = line.into();
                validate_line(&line)?;
                new_logline(line)
            }
        }

        impl<D: StructuredLine> TryFrom<$in> for Logline<Unvalidated, D> {
            type Error = &'static str;

            fn try_from(line: $in) -> Result<Self, Self::Error> {
                new_logline(line.into())
            }
        }
    };
}

impl_try_from!(&str);
impl_try_from!(String);
impl_try_from!(Box<str>);
impl_try_from!(Arc<str>);

macro_rules! convert_validation_state {
    ($from:ty, $to:ty) => {
        impl<D: StructuredLine> From<Logline<$from, D>> for Logline<$to, D> {
            fn from(line: Logline<$from, D>) -> Self {
                Self {
                    inner: line.inner,
                    _marker: PhantomData,
                }
            }
        }
    };
}

convert_validation_state!(Validated, Unvalidated);
convert_validation_state!(Unvalidated, Validated);
