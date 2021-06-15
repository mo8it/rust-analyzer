//! This is a fixture we use for tests that need lang items.
//!
//! We want to include the minimal subset of core for each test, so this file
//! supports "conditional compilation". Tests use the following syntax to include minicore:
//!
//!  //- minicore: flag1, flag2
//!
//! We then strip all the code marked with other flags.
//!
//! Available flags:
//!     sized:
//!     slice:
//!     range:
//!     unsize: sized
//!     deref: sized
//!     coerce_unsized: unsize
//!     pin:
//!     future: pin
//!     option:
//!     result:

pub mod marker {
    // region:sized
    #[lang = "sized"]
    #[fundamental]
    #[rustc_specialization_trait]
    pub trait Sized {}
    // endregion:sized

    // region:unsize
    #[lang = "unsize"]
    pub trait Unsize<T: ?Sized> {}
    // endregion:unsize
}

pub mod ops {
    // region:coerce_unsized
    mod unsize {
        use crate::marker::Unsize;

        #[lang = "coerce_unsized"]
        pub trait CoerceUnsized<T: ?Sized> {}

        impl<'a, T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<&'a mut U> for &'a mut T {}
        impl<'a, 'b: 'a, T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<&'a U> for &'b mut T {}
        impl<'a, T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<*mut U> for &'a mut T {}
        impl<'a, T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<*const U> for &'a mut T {}

        impl<'a, 'b: 'a, T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<&'a U> for &'b T {}
        impl<'a, T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<*const U> for &'a T {}

        impl<T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<*mut U> for *mut T {}
        impl<T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<*const U> for *mut T {}
        impl<T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<*const U> for *const T {}
    }
    pub use self::unsize::CoerceUnsized;
    // endregion:coerce_unsized

    // region:deref
    mod deref {
        #[lang = "deref"]
        pub trait Deref {
            #[lang = "deref_target"]
            type Target: ?Sized;
            fn deref(&self) -> &Self::Target;
        }
    }
    pub use self::deref::Deref;
    // endregion:deref

    // region:range
    mod range {
        #[lang = "RangeFull"]
        pub struct RangeFull;

        #[lang = "Range"]
        pub struct Range<Idx> {
            pub start: Idx,
            pub end: Idx,
        }

        #[lang = "RangeFrom"]
        pub struct RangeFrom<Idx> {
            pub start: Idx,
        }

        #[lang = "RangeTo"]
        pub struct RangeTo<Idx> {
            pub end: Idx,
        }

        #[lang = "RangeInclusive"]
        pub struct RangeInclusive<Idx> {
            pub(crate) start: Idx,
            pub(crate) end: Idx,
            pub(crate) exhausted: bool,
        }

        #[lang = "RangeToInclusive"]
        pub struct RangeToInclusive<Idx> {
            pub end: Idx,
        }
    }
    pub use self::range::{Range, RangeFrom, RangeFull, RangeTo};
    pub use self::range::{RangeInclusive, RangeToInclusive};
    // endregion:range
}

// region:slice
pub mod slice {
    #[lang = "slice"]
    impl<T> [T] {
        pub fn len(&self) -> usize {
            loop {}
        }
    }
}
// endregion:slice

// region:option
pub mod option {
    pub enum Option<T> {
        #[lang = "None"]
        None,
        #[lang = "Some"]
        Some(T),
    }
}
// endregion:option

// region:result
pub mod result {
    pub enum Result<T, E> {
        #[lang = "Ok"]
        Ok(T),
        #[lang = "Err"]
        Err(E),
    }
}
// endregion:result

// region:pin
pub mod pin {
    #[lang = "pin"]
    #[fundamental]
    pub struct Pin<P> {
        pointer: P,
    }
}
// endregion:pin

// region:future
pub mod future {
    use crate::{
        pin::Pin,
        task::{Context, Poll},
    };

    #[lang = "future_trait"]
    pub trait Future {
        type Output;
        #[lang = "poll"]
        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
    }
}
pub mod task {
    pub enum Poll<T> {
        #[lang = "Ready"]
        Ready(T),
        #[lang = "Pending"]
        Pending,
    }

    pub struct Context<'a> {
        waker: &'a (),
    }
}
// endregion:future

pub mod prelude {
    pub mod v1 {
        pub use crate::{
            marker::Sized,                      // :sized
            option::Option::{self, None, Some}, // :option
            result::Result::{self, Err, Ok},    // :result
        };
    }

    pub mod rust_2015 {
        pub use super::v1::*;
    }

    pub mod rust_2018 {
        pub use super::v1::*;
    }

    pub mod rust_2021 {
        pub use super::v1::*;
    }
}

#[prelude_import]
#[allow(unused)]
use prelude::v1::*;