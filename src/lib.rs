//! # Numbers [![GitHub Workflow Status](https://img.shields.io/github/workflow/status/unic0rn9k/num/Rust?label=tests&logo=github)](https://github.com/unic0rn9k/num/actions/workflows/rust.yml)
//!
//! # Example
//!
//! Bellow an example of eulers identity is shown.
//!
//! e^(i * pi) = -1
//!
//! ```rust
//! use num::*;
//!
//! assert_eq!(
//!     im(std::f32::consts::PI).exp_().re,
//!     -1.
//! );
//! ```
//!
//! # Thanks
//! Heavy inspiration has been taken from crates listed bellow.
//!
//! - [lib.rs/num](https://lib.rs/crates/num)
//! - [lib.rs/fast-floats](https://lib.rs/crates/fast-floats)

#![feature(const_trait_impl, core_intrinsics, const_fn_trait_bound)]

#[macro_export]
macro_rules! num {
    (0) => {
        <_>::_0
    };
    (1) => {
        <_>::_1
    };
    (2) => {
        <_>::_2
    };
    ($f: literal) => {
        <_>::from_f64($f)
    };
}

use std::{
    fmt::{Debug, Display},
    ops::*,
};

pub trait PrimitiveFloat: Float + FloatWrapper<InnerFloat = Self> {}

pub trait FloatWrapper {
    type InnerFloat: Float;
    fn from_primitive(f: Self::InnerFloat) -> Self;
    fn into_primitive(self) -> Self::InnerFloat;
    fn from_f64(f: f64) -> Self;
}

pub trait Float:
    Add<Self, Output = Self>
    + Div<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Copy
    + FloatWrapper
    + PartialEq
{
    const _0: Self;
    const _1: Self;
    const _2: Self;

    fn sqrt_(self) -> Self;
    fn powi_(self, p: i32) -> Self;
    fn hypot_(self, other: Self) -> Self;
    fn exp_(self) -> Self;
    fn sin_(self) -> Self;
    fn cos_(self) -> Self;
}

macro_rules! impl_wrapper {
	($fn: ident $( ($($arg: ident $({$($pre: tt)*})? : $t: ty),*) )? { $($finalize: tt)* } -> $re: ty) => {
		paste!{
            #[inline(always)]
            fn [< $fn _ >] (self $(, $($arg: $t),* )? ) -> $re {
                self . $fn($($( $arg $($($pre)*)? ),*)?) $($finalize)*
            }
        }
	};
}

macro_rules! def_primitive {
    ($t: ty) => {
        impl const FloatWrapper for $t {
            type InnerFloat = $t;

            #[inline(always)]
            fn from_primitive(f: Self::InnerFloat) -> Self {
                f
            }

            #[inline(always)]
            fn into_primitive(self) -> Self::InnerFloat {
                self
            }

            #[inline(always)]
            fn from_f64(f: f64) -> Self {
                f as $t
            }
        }

        impl const PrimitiveFloat for $t {}

        impl Float for $t {
            const _0: $t = 0.;
            const _1: $t = 1.;
            const _2: $t = 2.;

            impl_wrapper!(powi (p: i32) { .into() } -> Self);
            impl_wrapper!(hypot (other: Self) { .into() } -> Self);

            impl_wrapper!(sqrt { .into() } -> Self);
            impl_wrapper!(exp { .into() } -> Self);
            impl_wrapper!(sin { .into() } -> Self);
            impl_wrapper!(cos { .into() } -> Self);
        }
    };
}

def_primitive!(f32);
def_primitive!(f64);

#[cfg(feature = "fast-floats")]
pub mod fast_floats;
#[cfg(feature = "fast-floats")]
pub mod _fast_floats {
    use super::*;
    use fast_floats::*;

    macro_rules! def_fast_primitive {
        ($t: ty) => {
            impl const FloatWrapper for Fast<$t> {
                type InnerFloat = $t;

                #[inline(always)]
                fn from_primitive(f: Self::InnerFloat) -> Self {
                    unsafe{ Self::new(f) }
                }

                #[inline(always)]
                fn into_primitive(self) -> Self::InnerFloat {
                    *self
                }

                #[inline(always)]
                fn from_f64(f: f64) -> Self {
                    unsafe{ Self::new(f as $t) }
                }
            }

            impl Float for Fast<$t> {
                const _0: Self = unsafe{ Fast::new(0.) };
                const _1: Self = unsafe{ Fast::new(1.) };
                const _2: Self = unsafe{ Fast::new(2.) };

                impl_wrapper!(powi (p: i32) { .into() } -> Self);
                impl_wrapper!(hypot (other { .deref().clone() } : Self) { .into() } -> Self);

                impl_wrapper!(sqrt { .into() } -> Self);
                impl_wrapper!(exp { .into() } -> Self);
                impl_wrapper!(sin { .into() } -> Self);
                impl_wrapper!(cos { .into() } -> Self);
            }
        };
    }

    def_fast_primitive!(f32);
    def_fast_primitive!(f64);
}

macro_rules! impl_complex_plus_min {
    ($op: tt, $trait: ident, $fn: ident) => {
        impl<T: Float> $trait<Self> for Complex<T> {
            type Output = Self;
            fn $fn(self, other: Self) -> Self {
                Self::Output{re: self.re $op other.re, im: self.im $op other.im}
            }
        }
    };
}

impl_complex_plus_min!(+, Add, add);
impl_complex_plus_min!(-, Sub, sub);

impl<T: Float> Mul<Self> for Complex<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let re = self.re * other.re - self.im * other.im;
        let im = self.re * other.im + self.im * other.re;
        Self::Output { re, im }
    }
}
impl<T: Float> Div<Self> for Complex<T> {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        let re = self.re * other.re + self.im * other.im;
        let im = self.re * other.im - self.im * other.re;
        Self::Output { re, im }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Complex<T: Float> {
    pub re: T,
    pub im: T,
}

impl<T: Display + Float> Display for Complex<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} + {}im", self.re, self.im)
    }
}

impl<T: Float> From<T> for Complex<T> {
    fn from(n: T) -> Self {
        Self { re: n, im: num!(0) }
    }
}

impl<T: Float> FloatWrapper for Complex<T> {
    type InnerFloat = T;

    fn from_primitive(f: Self::InnerFloat) -> Self {
        Self { re: f, im: num!(0) }
    }

    fn into_primitive(self) -> T {
        //eprint!("Converting complex number to primitive should be done with self.re, instead of self.into_primitive.");
        self.re
    }

    fn from_f64(f: f64) -> Self {
        Self {
            re: T::from_f64(f),
            im: num!(0),
        }
    }
}

impl<T: Float> Float for Complex<T> {
    const _0: Self = Self {
        re: T::_0,
        im: T::_0,
    };
    const _1: Self = Self {
        re: T::_1,
        im: T::_0,
    };
    const _2: Self = Self {
        re: T::_2,
        im: T::_0,
    };

    fn powi_(self, n: i32) -> Self {
        let mut prod = Self::_1;
        for _ in 0..n {
            prod = prod * self
        }
        prod
    }

    fn sqrt_(self) -> Self {
        let norm = self.re.hypot_(self.im);
        Self {
            re: ((norm + self.re) / num!(2)).sqrt_(),
            im: ((norm - self.re) / num!(2)).sqrt_(),
        }
    }

    fn hypot_(self, other: Self) -> Self {
        (self * self + other * other).sqrt_()
    }

    fn exp_(self) -> Self {
        (re(self.im.cos_()) + im(self.im.sin_())) * re(self.re.exp_())
    }

    fn cos_(self) -> Self {
        let mut neg_self = self;
        neg_self.re = self.re * num!(-1.);
        (self.exp_() + (neg_self).exp_()) / num!(2)
    }

    fn sin_(self) -> Self {
        let mut neg_self = self;
        neg_self.re = self.re * num!(-1.);
        (self.exp_() - (neg_self).exp_()) / (Self::_2 * self.re.into())
    }
}

impl<T: Float> From<[T; 2]> for Complex<T> {
    fn from(n: [T; 2]) -> Self {
        Complex { re: n[0], im: n[1] }
    }
}

use paste::paste;
macro_rules! gen_tests {
    ($t: ty, $name: ident) => {
        paste! {
            #[cfg(test)]
            #[test]
            fn $name() {
                for x in 0..100 {
                    let x = $t::from_f64(x as f64) / 2.;
                    for y in 0..100 {
                        let y = $t::from_f64(y as f64) / 2.;
                        assert_eq!(x.hypot(y.into_primitive()), x.hypot_(y).into_primitive());
                    }
                    for n in 0..4 {
                        assert_eq!(x.powi(n), x.powi_(n).into_primitive());
                    }
                    assert_eq!(x.sqrt(), x.sqrt_().into_primitive());
                }
            }
        }
    };
}

gen_tests!(f32, trig_f32);
gen_tests!(f64, trig_f64);

#[cfg(test)]
use fast_floats::Fast;
gen_tests!(Fast::<f32>, trig_fast_f32);
gen_tests!(Fast::<f64>, trig_fast_f64);

/// Same as `Complex{re: 0., im: f}`
pub const fn im<F: Float + Sized>(f: F) -> Complex<F> {
    Complex { re: num!(0), im: f }
}

/// Same as `Complex{re: f, im: 0.}`
pub const fn re<F: Float + Sized>(f: F) -> Complex<F> {
    Complex { re: f, im: num!(0) }
}

#[cfg(feature = "fast-floats")]
/// Same as `unsafe{ Fast::new(f) }`
pub const fn fast<F: Float + Sized>(f: F) -> fast_floats::Fast<F> {
    unsafe { fast_floats::Fast::new(f) }
}
